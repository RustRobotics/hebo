// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use codec::PublishPacket;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::commands::{
    DispatcherToListenerCmd, DispatcherToMetricsCmd, ListenerToDispatcherCmd,
    MetricsToDispatcherCmd,
};
use crate::types::ListenerId;

/// Dispatcher is a message router.
#[derive(Debug)]
pub struct Dispatcher {
    listener_senders: Vec<(ListenerId, Sender<DispatcherToListenerCmd>)>,
    listener_receiver: Receiver<ListenerToDispatcherCmd>,

    metrics_sender: Sender<DispatcherToMetricsCmd>,
    metrics_receiver: Receiver<MetricsToDispatcherCmd>,
}

impl Dispatcher {
    pub fn new(
        listener_senders: Vec<(ListenerId, Sender<DispatcherToListenerCmd>)>,
        listener_receiver: Receiver<ListenerToDispatcherCmd>,
        metrics_sender: Sender<DispatcherToMetricsCmd>,
        metrics_receiver: Receiver<MetricsToDispatcherCmd>,
    ) -> Self {
        Dispatcher {
            listener_senders,
            listener_receiver,

            metrics_sender,
            metrics_receiver,
        }
    }

    pub async fn run_loop(&mut self) -> ! {
        loop {
            tokio::select! {
                Some(cmd) = self.listener_receiver.recv() => {
                    self.handle_listener_cmd(cmd).await;
                },
                Some(cmd) = self.metrics_receiver.recv() => {
                    self.handle_metrics_cmd(cmd).await;
                }
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
                self.storage_store_packet(&packet).await;
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

    /// Send packet to storage.
    async fn storage_store_packet(&mut self, packet: &PublishPacket) {
        log::info!("store packet: {:?}", packet);
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

    // TODO(Shaohua): Remove method
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
}
