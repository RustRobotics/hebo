// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

//! Metrics app handler

use super::Dispatcher;
use crate::commands::{DispatcherToMetricsCmd, MetricsToDispatcherCmd};
use crate::types::ListenerId;

impl Dispatcher {
    pub(super) async fn handle_metrics_cmd(&mut self, cmd: MetricsToDispatcherCmd) {
        match cmd {
            MetricsToDispatcherCmd::Publish(packet) => {
                self.publish_packet_to_sub_trie(&packet).await;
            }
            MetricsToDispatcherCmd::PublishV5(packet) => {
                self.publish_packet_to_sub_trie_v5(&packet).await;
            }
        }
    }

    #[allow(dead_code)]
    pub(super) async fn metrics_publish_packet_sent(
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

    pub(super) async fn metrics_on_session_added(&mut self, listener_id: ListenerId) {
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

    pub(super) async fn metrics_on_session_removed(&mut self, listener_id: ListenerId) {
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

    pub(super) async fn metrics_on_subscription_added(
        &mut self,
        listener_id: ListenerId,
        n: usize,
    ) {
        if let Err(err) = self
            .metrics_sender
            .send(DispatcherToMetricsCmd::SubscriptionsAdded(listener_id, n))
            .await
        {
            log::error!(
                "Dispatcher: Failed to send SubscriptionsAdded cmd, err: {:?}",
                err
            );
        }
    }

    pub(super) async fn metrics_on_subscription_removed(
        &mut self,
        listener_id: ListenerId,
        n: usize,
    ) {
        if let Err(err) = self
            .metrics_sender
            .send(DispatcherToMetricsCmd::SubscriptionsRemoved(listener_id, n))
            .await
        {
            log::error!(
                "Dispatcher: Failed to send SubscriptionsRemoved cmd, err: {:?}",
                err
            );
        }
    }
}
