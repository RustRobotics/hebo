// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use codec::{PublishPacket, SubscribePacket, UnsubscribePacket};

use super::Dispatcher;
use crate::commands::{DispatcherToListenerCmd, ListenerToDispatcherCmd};
use crate::types::SessionGid;

impl Dispatcher {
    pub(super) async fn handle_listener_cmd(&mut self, cmd: ListenerToDispatcherCmd) {
        log::info!("handle_listener_cmd: {:?}", cmd);
        match cmd {
            ListenerToDispatcherCmd::Publish(packet) => {
                self.backends_store_packet(&packet).await;
                self.on_listener_publish(&packet).await;
            }
            ListenerToDispatcherCmd::Subscribe(session_gid, packet) => {
                self.on_listener_subscribe(session_gid, packet).await;
            }
            ListenerToDispatcherCmd::Unsubscribe(session_gid, packet) => {
                self.on_listener_unsubscribe(session_gid, packet).await;
            }
            ListenerToDispatcherCmd::SessionAdded(listener_id) => {
                self.metrics_on_session_added(listener_id).await;
            }
            ListenerToDispatcherCmd::SessionRemoved(listener_id) => {
                self.metrics_on_session_removed(listener_id).await;
            }
        }
    }

    pub(super) async fn on_listener_publish(&mut self, packet: &PublishPacket) {
        self.publish_packet_to_sub_trie(packet).await;
    }

    async fn on_listener_subscribe(&mut self, session_gid: SessionGid, packet: SubscribePacket) {
        let (sub_ack_packet, n_subscribed) = self.sub_trie.subscribe(session_gid, packet);

        self.metrics_on_subscription_added(session_gid.listener_id(), n_subscribed)
            .await;

        if let Some(listener_sender) = self.listener_senders.get(&session_gid.listener_id()) {
            let cmd =
                DispatcherToListenerCmd::SubscribeAck(session_gid.session_id(), sub_ack_packet);
            if let Err(err) = listener_sender.send(cmd).await {
                log::error!(
                    "dispatcher: Failed to send subscribe ack to listener: {:?}, err: {:?}",
                    session_gid,
                    err
                );
            }
        } else {
            log::error!(
                "dispatcher: Failed to find listener sender with id: {}",
                session_gid.listener_id()
            );
        }
    }

    async fn on_listener_unsubscribe(
        &mut self,
        session_gid: SessionGid,
        packet: UnsubscribePacket,
    ) {
        let n_unsubscribed = self.sub_trie.unsubscribe(session_gid, packet);
        self.metrics_on_subscription_removed(session_gid.listener_id(), n_unsubscribed)
            .await;
    }
}
