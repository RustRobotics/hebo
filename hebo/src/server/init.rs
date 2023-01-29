// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

//! Init server context internal modules and apps.

use tokio::runtime::Runtime;
use tokio::sync::mpsc;

use super::{ServerContext, CHANNEL_CAPACITY};
use crate::auth::AuthApp;
use crate::backends::BackendsApp;
use crate::bridge::BridgeApp;
use crate::commands::DispatcherToMetricsCmd;
use crate::dispatcher::Dispatcher;
use crate::error::Error;
use crate::gateway::GatewayApp;
use crate::listener::Listener;
use crate::metrics::Metrics;

#[cfg(feature = "acl")]
use crate::acl::AclApp;
#[cfg(feature = "dashboard")]
use crate::dashboard::DashboardApp;
#[cfg(feature = "rule_engine")]
use crate::rule_engine::RuleEngineApp;

impl ServerContext {
    #[allow(clippy::too_many_lines)]
    pub(crate) async fn init_modules(&mut self, runtime: &Runtime) -> Result<(), Error> {
        log::info!("ServerContext::init_modules()");

        let (listeners_to_dispatcher_sender, listeners_to_dispatcher_receiver) =
            mpsc::channel(CHANNEL_CAPACITY);
        let mut dispatcher_to_listener_senders = Vec::new();
        let (listeners_to_auth_sender, listeners_to_auth_receiver) =
            mpsc::channel(CHANNEL_CAPACITY);
        let mut auth_to_listener_senders = Vec::new();
        let (listeners_to_acl_sender, listeners_to_acl_receiver) = mpsc::channel(CHANNEL_CAPACITY);
        let mut acl_to_listener_senders = Vec::new();

        let mut handles = Vec::new();
        let mut listeners_info = Vec::new();

        // Listeners module.
        let mut listener_objs = Vec::new();
        for (listener_id, l) in (0_u32..).zip(self.config.listeners().iter()) {
            listeners_info.push((listener_id, l.address()));
            let (dispatcher_to_listener_sender, dispatcher_to_listener_receiver) =
                mpsc::channel(CHANNEL_CAPACITY);
            dispatcher_to_listener_senders.push((listener_id, dispatcher_to_listener_sender));

            let (auth_to_listener_sender, auth_to_listener_receiver) =
                mpsc::channel(CHANNEL_CAPACITY);
            auth_to_listener_senders.push((listener_id, auth_to_listener_sender));

            let (acl_to_listener_sender, acl_to_listener_receiver) =
                mpsc::channel(CHANNEL_CAPACITY);
            acl_to_listener_senders.push((listener_id, acl_to_listener_sender));

            let listener = Listener::bind(
                listener_id,
                l.clone(),
                // dispatcher module
                listeners_to_dispatcher_sender.clone(),
                dispatcher_to_listener_receiver,
                // Auth module
                listeners_to_auth_sender.clone(),
                auth_to_listener_receiver,
                // acl module
                listeners_to_acl_sender.clone(),
                acl_to_listener_receiver,
            )
                .await
                .unwrap_or_else(|_| panic!("Failed to listen at {:?}", &listeners_info.last()));
            listener_objs.push(listener);
        }

        self.set_uid()?;

        for mut listener in listener_objs {
            let handle = runtime.spawn(async move {
                listener.run_loop().await;
            });
            handles.push(handle);
        }

        // Metrics module.
        let (metrics_to_dispatcher_sender, metrics_to_dispatcher_receiver) =
            mpsc::channel(CHANNEL_CAPACITY);
        let (dispatcher_to_metrics_sender, dispatcher_to_metrics_receiver) =
            mpsc::channel(CHANNEL_CAPACITY);
        let mut metrics = Metrics::new(
            self.config.general().sys_interval(),
            metrics_to_dispatcher_sender,
            dispatcher_to_metrics_receiver,
            // server ctx
            self.metrics_receiver.take().unwrap(),
        );
        let metrics_handle = runtime.spawn(async move {
            metrics.run_loop().await;
        });
        handles.push(metrics_handle);

        for listener_info in &listeners_info {
            if let Err(err) = dispatcher_to_metrics_sender
                .send(DispatcherToMetricsCmd::ListenerAdded(
                    listener_info.0,
                    listener_info.1.to_string(),
                ))
                .await
            {
                log::error!(
                    "Failed to send listener {:?} to metrics, err: {:?}",
                    listener_info.1,
                    err
                );
            }
        }

        // Auth module.
        let mut auth_app = AuthApp::new(
            self.config.security(),
            // listeners
            auth_to_listener_senders,
            listeners_to_auth_receiver,
            // server ctx
            self.auth_receiver.take().unwrap(),
        )?;
        let auth_app_handle = runtime.spawn(async move {
            auth_app.run_loop().await;
        });
        handles.push(auth_app_handle);

        #[cfg(feature = "acl")]
        {
            // ACL module.
            let mut acl_app = AclApp::new(
                // listeners
                acl_to_listener_senders,
                listeners_to_acl_receiver,
                // server ctx
                self.acl_receiver.take().unwrap(),
            );
            let acl_app_handle = runtime.spawn(async move {
                acl_app.run_loop().await;
            });
            handles.push(acl_app_handle);
        }

        #[cfg(not(feature = "acl"))]
        {
            drop(listeners_to_acl_receiver);
        }

        // Backends module.
        let (backends_to_dispatcher_sender, backends_to_dispatcher_receiver) =
            mpsc::channel(CHANNEL_CAPACITY);
        let (dispatcher_to_backends_sender, dispatcher_to_backends_receiver) =
            mpsc::channel(CHANNEL_CAPACITY);
        let mut backends_app = BackendsApp::new(
            // dispatcher
            backends_to_dispatcher_sender,
            dispatcher_to_backends_receiver,
            // server ctx
            self.backends_receiver.take().unwrap(),
        );
        let backends_handle = runtime.spawn(async move {
            backends_app.run_loop().await;
        });
        handles.push(backends_handle);

        // bridge module.
        let (bridge_to_dispatcher_sender, bridge_to_dispatcher_receiver) =
            mpsc::channel(CHANNEL_CAPACITY);
        let (dispatcher_to_bridge_sender, dispatcher_to_bridge_receiver) =
            mpsc::channel(CHANNEL_CAPACITY);
        let mut bridge_app = BridgeApp::new(
            // dispatcher
            bridge_to_dispatcher_sender,
            dispatcher_to_bridge_receiver,
            // server ctx
            self.bridge_receiver.take().unwrap(),
        );
        let bridge_handle = runtime.spawn(async move {
            bridge_app.run_loop().await;
        });
        handles.push(bridge_handle);

        // dashboard module.
        #[cfg(feature = "dashboard")]
        if self.config.dashboard().enable() {
            let mut dashboard_app = DashboardApp::new(
                self.config.dashboard(),
                // server ctx
                self.dashboard_sender.take().unwrap(),
            )?;
            let dashboard_handle = runtime.spawn(async move {
                dashboard_app.run_loop().await;
            });
            handles.push(dashboard_handle);
        }

        // gateway module.
        let (gateway_to_dispatcher_sender, gateway_to_dispatcher_receiver) =
            mpsc::channel(CHANNEL_CAPACITY);
        let (dispatcher_to_gateway_sender, dispatcher_to_gateway_receiver) =
            mpsc::channel(CHANNEL_CAPACITY);
        let mut gateway_app = GatewayApp::new(
            // dispatcher
            gateway_to_dispatcher_sender,
            dispatcher_to_gateway_receiver,
            // server ctx
            self.gateway_receiver.take().unwrap(),
        );
        let gateway_handle = runtime.spawn(async move {
            gateway_app.run_loop().await;
        });
        handles.push(gateway_handle);

        // rule engine module.
        let (rule_engine_to_dispatcher_sender, rule_engine_to_dispatcher_receiver) =
            mpsc::channel(CHANNEL_CAPACITY);
        let (dispatcher_to_rule_engine_sender, dispatcher_to_rule_engine_receiver) =
            mpsc::channel(CHANNEL_CAPACITY);

        #[cfg(feature = "rule_engine")]
        {
            let mut rule_engine_app = RuleEngineApp::new(
                // dispatcher
                rule_engine_to_dispatcher_sender,
                dispatcher_to_rule_engine_receiver,
                // server ctx
                self.rule_engine_receiver.take().unwrap(),
            );
            let rule_engine_handle = runtime.spawn(async move {
                rule_engine_app.run_loop().await;
            });
            handles.push(rule_engine_handle);
        }
        #[cfg(not(feature = "rule_engine"))]
        {
            drop(rule_engine_to_dispatcher_sender);
            drop(dispatcher_to_rule_engine_receiver);
        }

        // Dispatcher module.
        let mut dispatcher = Dispatcher::new(
            // backends module
            dispatcher_to_backends_sender,
            backends_to_dispatcher_receiver,
            // bridge module
            dispatcher_to_bridge_sender,
            bridge_to_dispatcher_receiver,
            // gateway module
            dispatcher_to_gateway_sender,
            gateway_to_dispatcher_receiver,
            // metrics module
            dispatcher_to_metrics_sender,
            metrics_to_dispatcher_receiver,
            // listeners module
            dispatcher_to_listener_senders,
            listeners_to_dispatcher_receiver,
            // rule engine module
            dispatcher_to_rule_engine_sender,
            rule_engine_to_dispatcher_receiver,
        );
        let dispatcher_handle = runtime.spawn(async move {
            dispatcher.run_loop().await;
        });
        handles.push(dispatcher_handle);

        Ok(())
    }
}
