// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use codec::PublishPacket;

use super::Dispatcher;
use crate::commands::{DispatcherToListenerCmd, ListenerToDispatcherCmd};

impl Dispatcher {
    pub(super) async fn handle_listener_cmd(&mut self, cmd: ListenerToDispatcherCmd) {
        log::info!("handle_listener_cmd: {:?}", cmd);
        match cmd {
            ListenerToDispatcherCmd::Publish(packet) => {
                self.backends_store_packet(&packet).await;
                self.publish_packet_to_listners(&packet).await;
            }
            ListenerToDispatcherCmd::Subscribe(session_gid, packet) => {
                unimplemented!();
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
    pub(super) async fn publish_packet_to_listners(&mut self, packet: &PublishPacket) {
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
}
