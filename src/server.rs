// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

//! ServerContex is the main entry pointer of hebo server.

use clap::Arg;
use std::fs::File;
use std::io::{Read, Write};
use tokio::runtime::Runtime;
use tokio::sync::{
    broadcast,
    mpsc::{self, Receiver, Sender},
    oneshot,
};

use crate::acl::app::AclApp;
use crate::auth::app::AuthApp;
use crate::backends::app::BackendsApp;
use crate::bridge::app::BridgeApp;
use crate::commands::{
    DashboardToServerContexCmd, DispatcherToMetricsCmd, ServerContextRequestCmd,
    ServerContextResponseCmd, ServerContextToAclCmd, ServerContextToAuthCmd,
    ServerContextToBackendsCmd, ServerContextToMetricsCmd,
};
use crate::config::Config;
use crate::dashboard::app::DashboardApp;
use crate::dispatcher::Dispatcher;
use crate::error::{Error, ErrorKind};
use crate::gateway::app::GatewayApp;
use crate::listener::Listener;
use crate::log::init_log;
use crate::metrics::Metrics;
use crate::rule_engine::app::RuleEngineApp;

pub const DEFAULT_CONFIG: &str = "/etc/hebo/hebo.toml";
pub const CHANNEL_CAPACITY: usize = 16;

/// ServerContext manages lifetime of Dispatcher and Listeners.
/// All kernel signals are handled here.
#[derive(Debug)]
pub struct ServerContext {
    config: Config,

    // TODO(Shaohua): Remove
    request_sender: broadcast::Sender<ServerContextRequestCmd>,
    request_receiver: broadcast::Receiver<ServerContextRequestCmd>,

    response_sender: Sender<ServerContextResponseCmd>,
    response_receiver: Receiver<ServerContextResponseCmd>,

    // dashboard -> server_ctx
    dashboard_sender: Option<Sender<DashboardToServerContexCmd>>,
    dashboard_receiver: Receiver<DashboardToServerContexCmd>,

    // server_ctx -> acl
    acl_sender: Sender<ServerContextToAclCmd>,
    acl_receiver: Option<Receiver<ServerContextToAclCmd>>,

    // server_ctx -> auth
    auth_sender: Sender<ServerContextToAuthCmd>,
    auth_receiver: Option<Receiver<ServerContextToAuthCmd>>,

    // server_ctx -> backends
    backends_sender: Sender<ServerContextToBackendsCmd>,
    backends_receiver: Option<Receiver<ServerContextToBackendsCmd>>,

    // server_ctx -> metrics
    metrics_sender: Sender<ServerContextToMetricsCmd>,
    metrics_receiver: Option<Receiver<ServerContextToMetricsCmd>>,
}

impl ServerContext {
    pub fn new(config: Config) -> ServerContext {
        // TODO(Shaohua): Remove
        // A broadcast channel connects server context to all apps.
        // So that these apps will receive commands from server context.
        let (request_sender, request_receiver) = broadcast::channel(CHANNEL_CAPACITY);

        // TODO(Shaohua): Remove
        // A mpsc channel is used to send response cmd from apps to server context.
        let (response_sender, response_receiver) = mpsc::channel(CHANNEL_CAPACITY);

        let (dashboard_sender, dashboard_receiver) = mpsc::channel(CHANNEL_CAPACITY);
        let (acl_sender, acl_receiver) = mpsc::channel(CHANNEL_CAPACITY);
        let (auth_sender, auth_receiver) = mpsc::channel(CHANNEL_CAPACITY);
        let (backends_sender, backends_receiver) = mpsc::channel(CHANNEL_CAPACITY);
        let (metrics_sender, metrics_receiver) = mpsc::channel(CHANNEL_CAPACITY);

        ServerContext {
            config,

            request_sender,
            request_receiver,

            response_sender,
            response_receiver,

            dashboard_sender: Some(dashboard_sender),
            dashboard_receiver,

            acl_sender,
            acl_receiver: Some(acl_receiver),

            auth_sender,
            auth_receiver: Some(auth_receiver),

            backends_sender,
            backends_receiver: Some(backends_receiver),

            metrics_sender,
            metrics_receiver: Some(metrics_receiver),
        }
    }

    /// Notify server process to reload config by sending `SIGUSR1` signal.
    pub fn reload(&mut self) -> Result<(), Error> {
        log::info!("reload()");
        let mut fd = File::open(&self.config.general.pid_file)?;
        let mut pid_str = String::new();
        fd.read_to_string(&mut pid_str)?;
        log::info!("pid str: {}", pid_str);
        let pid = pid_str.parse::<i32>().map_err(|err| {
            Error::from_string(
                ErrorKind::PidError,
                format!(
                    "Failed to parse pid {} from file {:?}, err: {:?}",
                    pid_str, &self.config.general.pid_file, err
                ),
            )
        })?;
        nc::kill(pid, nc::SIGUSR1).map_err(|err| {
            Error::from_string(
                ErrorKind::PidError,
                format!(
                    "Failed to notify process {}, got {}",
                    pid,
                    nc::strerror(err)
                ),
            )
        })?;
        Ok(())
    }

    fn write_pid(&self) -> Result<(), Error> {
        let pid = std::process::id();
        let mut fd = File::create(&self.config.general.pid_file)?;
        write!(fd, "{}", pid)?;
        Ok(())
    }

    /// Init modules and run tokio runtime.
    pub fn run_loop(&mut self, runtime: Runtime) -> Result<(), Error> {
        self.write_pid()?;

        runtime.block_on(async {
            self.init_modules(&runtime).await?;
            self.run_inner_loop().await
        })
    }

    async fn run_inner_loop(&mut self) -> Result<(), Error> {
        log::info!("ServerContext::run_inner_loop()");
        loop {
            tokio::select! {
                Some(cmd) = self.response_receiver.recv() => {
                    if let Err(err) = self.handle_response_cmd(cmd).await {
                        log::error!("Failed to handle response cmd: {:?}", err);
                    }
                },
                Some(cmd) = self.dashboard_receiver.recv() => {
                    if let Err(err) = self.handle_dashboard_cmd(cmd).await {
                        log::error!("Failed to handle dashboard cmd: {:?}", err);
                    }
                }
            }
        }

        // TODO(Shaohua): Break main loop
        #[allow(unreachable_code)]
        Ok(())
    }

    async fn handle_response_cmd(&mut self, cmd: ServerContextResponseCmd) -> Result<(), Error> {
        log::info!("cmd: {:?}", cmd);
        Ok(())
    }

    async fn handle_dashboard_cmd(&mut self, cmd: DashboardToServerContexCmd) -> Result<(), Error> {
        match cmd {
            DashboardToServerContexCmd::MetricsGetUptime(resp_tx) => {
                let (resp2_tx, resp2_rx) = oneshot::channel();

                self.metrics_sender
                    .send(ServerContextToMetricsCmd::MetricsGetUptime(resp2_tx))
                    .await?;
                let ret = resp2_rx.await?;
                resp_tx.send(ret).map_err(|_| {
                    Error::from_string(
                        ErrorKind::ChannelError,
                        format!("Failed to send metrics uptime to dashboard"),
                    )
                })
            }
        }
    }

    async fn init_modules(&mut self, runtime: &Runtime) -> Result<(), Error> {
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
        let mut listener_id: u32 = 0;
        let mut listeners_info = Vec::new();

        // Listeners module.
        for l in self.config.listeners.clone() {
            listeners_info.push((listener_id, l.address.clone()));
            let (dispatcher_to_listener_sender, dispatcher_to_listener_receiver) =
                mpsc::channel(CHANNEL_CAPACITY);
            dispatcher_to_listener_senders.push((listener_id, dispatcher_to_listener_sender));

            let (auth_to_listener_sender, auth_to_listener_receiver) =
                mpsc::channel(CHANNEL_CAPACITY);
            auth_to_listener_senders.push((listener_id, auth_to_listener_sender));

            let (acl_to_listener_sender, acl_to_listener_receiver) =
                mpsc::channel(CHANNEL_CAPACITY);
            acl_to_listener_senders.push((listener_id, acl_to_listener_sender));

            let mut listener = Listener::bind(
                listener_id,
                l,
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
            .expect(&format!("Failed to listen at {:?}", &listeners_info.last()));
            let handle = runtime.spawn(async move {
                listener.run_loop().await;
            });
            handles.push(handle);
            listener_id += 1;
        }

        // Metrics module.
        let (metrics_to_dispatcher_sender, metrics_to_dispatcher_receiver) =
            mpsc::channel(CHANNEL_CAPACITY);
        let (dispatcher_to_metrics_sender, dispatcher_to_metrics_receiver) =
            mpsc::channel(CHANNEL_CAPACITY);
        let mut metrics = Metrics::new(
            self.config.general.sys_interval,
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
                    listener_info.1.clone(),
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
            self.config.security.clone(),
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
            self.response_sender.clone(),
            self.request_sender.subscribe(),
        );
        let bridge_handle = runtime.spawn(async move {
            bridge_app.run_loop().await;
        });
        handles.push(bridge_handle);

        // dashboard module.
        let mut dashboard_app = DashboardApp::new(
            &self.config.dashboard,
            // server ctx
            self.dashboard_sender.take().unwrap(),
        )?;
        let dashboard_handle = runtime.spawn(async move {
            dashboard_app.run_loop().await;
        });
        handles.push(dashboard_handle);

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
            self.response_sender.clone(),
            self.request_sender.subscribe(),
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
        let mut rule_engine_app = RuleEngineApp::new(
            // dispatcher
            rule_engine_to_dispatcher_sender,
            dispatcher_to_rule_engine_receiver,
            // server ctx
            self.response_sender.clone(),
            self.request_sender.subscribe(),
        );
        let rule_engine_handle = runtime.spawn(async move {
            rule_engine_app.run_loop().await;
        });
        handles.push(rule_engine_handle);

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

/// Entry point of server
pub fn run_server() -> Result<(), Error> {
    let matches = clap::App::new("Hebo")
        .version("0.1.0")
        .author("Xu Shaohua <shaohua@biofan.org>")
        .about("High Performance MQTT Server")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("config_file")
                .takes_value(true)
                .help("Specify config file path"),
        )
        .arg(
            Arg::with_name("reload")
                .short("r")
                .long("reload")
                .takes_value(false)
                .help("Reload config"),
        )
        .arg(
            Arg::with_name("test")
                .short("t")
                .long("test")
                .takes_value(false)
                .help("Test config file"),
        )
        .get_matches();

    let config_file = matches.value_of("config").unwrap_or(DEFAULT_CONFIG);
    let config_content = std::fs::read_to_string(config_file)?;
    let config: Config = toml::from_str(&config_content).map_err(|err| {
        Error::from_string(ErrorKind::ConfigError, format!("Invalid config: {:?}", err))
    })?;

    if matches.is_present("test") {
        println!("The configuration file {} syntax is Ok", config_file);
        return Ok(());
    }

    init_log(&config.log)?;

    let mut server = ServerContext::new(config);

    if matches.is_present("reload") {
        log::info!("Reload is present");
        return server.reload();
    }

    let runtime = Runtime::new()?;
    server.run_loop(runtime)
}
