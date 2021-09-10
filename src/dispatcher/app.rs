// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use codec::PublishPacket;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::commands::{
    BackendsToDispatcherCmd, BridgeToDispatcherCmd, DispatcherToBackendsCmd, DispatcherToBridgeCmd,
    DispatcherToGatewayCmd, DispatcherToListenerCmd, DispatcherToMetricsCmd,
    DispatcherToRuleEngineCmd, GatewayToDispatcherCmd, ListenerToDispatcherCmd,
    MetricsToDispatcherCmd, RuleEngineToDispatcherCmd,
};
use crate::types::ListenerId;

/// Dispatcher is a message router.
#[derive(Debug)]
pub struct DispatcherApp {
    backends_sender: Sender<DispatcherToBackendsCmd>,
    backends_receiver: Receiver<BackendsToDispatcherCmd>,

    bridge_sender: Sender<DispatcherToBridgeCmd>,
    bridge_receiver: Receiver<BridgeToDispatcherCmd>,

    gateway_sender: Sender<DispatcherToGatewayCmd>,
    gateway_receiver: Receiver<GatewayToDispatcherCmd>,

    metrics_sender: Sender<DispatcherToMetricsCmd>,
    metrics_receiver: Receiver<MetricsToDispatcherCmd>,

    listener_senders: Vec<(ListenerId, Sender<DispatcherToListenerCmd>)>,
    listener_receiver: Receiver<ListenerToDispatcherCmd>,

    rule_engine_sender: Sender<DispatcherToRuleEngineCmd>,
    rule_engine_receiver: Receiver<RuleEngineToDispatcherCmd>,
}

impl DispatcherApp {
    pub fn new(
        backends_sender: Sender<DispatcherToBackendsCmd>,
        backends_receiver: Receiver<BackendsToDispatcherCmd>,

        bridge_sender: Sender<DispatcherToBridgeCmd>,
        bridge_receiver: Receiver<BridgeToDispatcherCmd>,

        gateway_sender: Sender<DispatcherToGatewayCmd>,
        gateway_receiver: Receiver<GatewayToDispatcherCmd>,

        metrics_sender: Sender<DispatcherToMetricsCmd>,
        metrics_receiver: Receiver<MetricsToDispatcherCmd>,

        listener_senders: Vec<(ListenerId, Sender<DispatcherToListenerCmd>)>,
        listener_receiver: Receiver<ListenerToDispatcherCmd>,

        rule_engine_sender: Sender<DispatcherToRuleEngineCmd>,
        rule_engine_receiver: Receiver<RuleEngineToDispatcherCmd>,
    ) -> Self {
        Dispatcher {
            backends_sender,
            backends_receiver,

            bridge_sender,
            bridge_receiver,

            gateway_sender,
            gateway_receiver,

            metrics_sender,
            metrics_receiver,

            listener_senders,
            listener_receiver,

            rule_engine_sender,
            rule_engine_receiver,
        }
    }

    pub async fn run_loop(&mut self) -> ! {
        loop {
            tokio::select! {
                Some(cmd) = self.backends_receiver.recv() => {
                    self.handle_backends_cmd(cmd).await;
                }
                Some(cmd) = self.bridge_receiver.recv() => {
                    self.handle_bridge_cmd(cmd).await;
                }
                Some(cmd) = self.gateway_receiver.recv() => {
                    self.handle_gateway_cmd(cmd).await;
                }
                Some(cmd) = self.metrics_receiver.recv() => {
                    self.handle_metrics_cmd(cmd).await;
                }
                Some(cmd) = self.listener_receiver.recv() => {
                    self.handle_listener_cmd(cmd).await;
                },
                Some(cmd) = self.rule_engine_receiver.recv() => {
                    self.handle_rule_engine_cmd(cmd).await;
                },
            }
        }
    }

    async fn handle_metrics_cmd(&mut self, cmd: MetricsToDispatcherCmd) {
        log::info!("handle metrics cmd: {:?}", cmd);
        match cmd {
            MetricsToDispatcherCmd::Publish(packet) => {
                self.publish_packet_to_listners(&packet).await;
            }
        }
    }

    async fn handle_listener_cmd(&mut self, cmd: ListenerToDispatcherCmd) {
        log::info!("handle_listener_cmd: {:?}", cmd);
        match cmd {
            ListenerToDispatcherCmd::Publish(packet) => {
                self.backends_store_packet(&packet).await;
                self.publish_packet_to_listners(&packet).await;
            }
            ListenerToDispatcherCmd::SessionAdded(listener_id) => {
                self.metrics_on_session_added(listener_id).await;
            }
            ListenerToDispatcherCmd::SessionRemoved(listener_id) => {
                self.metrics_on_session_removed(listener_id).await;
            }
            ListenerToDispatcherCmd::SubscriptionsAdded(listener_id) => {
                self.metrics_on_subscription_added(listener_id).await;
            }
            ListenerToDispatcherCmd::SubscriptionsRemoved(listener_id) => {
                self.metrics_on_subscription_removed(listener_id).await;
            }
        }
    }

    /// Send packet to backends.
    async fn backends_store_packet(&mut self, packet: &PublishPacket) {
        log::info!("backends store packet: {:?}", packet);
    }

    async fn publish_packet_to_listners(&mut self, packet: &PublishPacket) {
        for (_listener_id, sender) in &self.listener_senders {
            let cmd = DispatcherToListenerCmd::Publish(packet.clone());
            if let Err(err) = sender.send(cmd).await {
                log::error!(
                    "Dispatcher::publish_packet_to_listener() send failed: {:?}",
                    err
                );
            }
        }
    }

    // Metrics related methods
    async fn metrics_publish_packet_sent(
        &mut self,
        listener_id: ListenerId,
        count: usize,
        bytes: usize,
    ) {
        if let Err(err) = self
            .metrics_sender
            .send(DispatcherToMetricsCmd::PublishPacketSent(
                listener_id,
                count,
                bytes,
            ))
            .await
        {
            log::error!(
                "Dispatcher: Failed to send UpdatePublishPacket, err: {:?}",
                err
            );
        }
    }

    async fn metrics_on_session_added(&mut self, listener_id: ListenerId) {
        if let Err(err) = self
            .metrics_sender
            .send(DispatcherToMetricsCmd::SessionAdded(listener_id, 1))
            .await
        {
            log::error!(
                "Dispatcher: Failed to send SessionAdded cmd, err: {:?}",
                err
            );
        }
    }

    async fn metrics_on_session_removed(&mut self, listener_id: ListenerId) {
        if let Err(err) = self
            .metrics_sender
            .send(DispatcherToMetricsCmd::SessionRemoved(listener_id, 1))
            .await
        {
            log::error!(
                "Dispatcher: Failed to send SessionRemoved cmd, err: {:?}",
                err
            );
        }
    }

    async fn metrics_on_subscription_added(&mut self, listener_id: ListenerId) {
        if let Err(err) = self
            .metrics_sender
            .send(DispatcherToMetricsCmd::SubscriptionsAdded(listener_id, 1))
            .await
        {
            log::error!(
                "Dispatcher: Failed to send SubscriptionsAdded cmd, err: {:?}",
                err
            );
        }
    }

    async fn metrics_on_subscription_removed(&mut self, listener_id: ListenerId) {
        if let Err(err) = self
            .metrics_sender
            .send(DispatcherToMetricsCmd::SubscriptionsRemoved(listener_id, 1))
            .await
        {
            log::error!(
                "Dispatcher: Failed to send SubscriptionsRemoved cmd, err: {:?}",
                err
            );
        }
    }

    /// Backends app handlers
    async fn handle_backends_cmd(&mut self, cmd: BackendsToDispatcherCmd) {
        log::info!("cmd: {:?}", cmd);
    }

    /// Bridge app handlers
    async fn handle_bridge_cmd(&mut self, cmd: BridgeToDispatcherCmd) {
        log::info!("cmd: {:?}", cmd);
    }

    /// Gateway app handler
    async fn handle_gateway_cmd(&mut self, cmd: GatewayToDispatcherCmd) {
        log::info!("cmd: {:?}", cmd);
    }

    /// RuleEngine app handler
    async fn handle_rule_engine_cmd(&mut self, cmd: RuleEngineToDispatcherCmd) {
        log::info!("cmd: {:?}", cmd);
    }
}
