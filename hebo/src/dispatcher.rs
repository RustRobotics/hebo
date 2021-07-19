// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use codec::PublishPacket;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::commands::{
    CacheToDispatcherCmd, DispatcherToCacheCmd, DispatcherToListenerCmd, ListenerId,
    ListenerToDispatcherCmd, SystemToDispatcherCmd,
};

/// Dispatcher is a message router.
#[derive(Debug)]
pub struct Dispatcher {
    listener_senders: Vec<(u32, Sender<DispatcherToListenerCmd>)>,
    listener_receiver: Receiver<ListenerToDispatcherCmd>,

    system_receiver: Receiver<SystemToDispatcherCmd>,

    cache_sender: Sender<DispatcherToCacheCmd>,
    cache_receiver: Receiver<CacheToDispatcherCmd>,
}

impl Dispatcher {
    pub fn new(
        listener_senders: Vec<(u32, Sender<DispatcherToListenerCmd>)>,
        listener_receiver: Receiver<ListenerToDispatcherCmd>,
        system_receiver: Receiver<SystemToDispatcherCmd>,
        cache_sender: Sender<DispatcherToCacheCmd>,
        cache_receiver: Receiver<CacheToDispatcherCmd>,
    ) -> Self {
        Dispatcher {
            listener_senders,
            listener_receiver,

            system_receiver,

            cache_sender,
            cache_receiver,
        }
    }

    pub async fn run_loop(&mut self) -> ! {
        loop {
            tokio::select! {
                Some(cmd) = self.listener_receiver.recv() => {
                    self.handle_listener_cmd(cmd).await;
                },
                Some(cmd) = self.system_receiver.recv() => {
                    self.handle_system_cmd(cmd).await;
                }
            }
        }
    }

    async fn handle_system_cmd(&mut self, cmd: SystemToDispatcherCmd) {
        log::info!("handle system cmd: {:?}", cmd);
        match cmd {
            SystemToDispatcherCmd::Publish(packet) => {
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
            ListenerToDispatcherCmd::SessionAdded(listener_id, _session_id) => {
                self.cache_on_session_added(listener_id).await;
            }
            ListenerToDispatcherCmd::SessionRemoved(listener_id, _session_id) => {
                self.cache_on_session_removed(listener_id).await;
            }
            ListenerToDispatcherCmd::SubscriptionsAdded(listener_id, _session_id) => {
                self.cache_on_subscription_added(listener_id).await;
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
                log::error!("Dispatcher::handle_listener_cmd() send failed: {:?}", err);
            }
        }
    }

    //
    // Cache related methods
    //
    async fn cache_publish_packet_sent(&mut self, listener_id: u32, count: usize, bytes: usize) {
        if let Err(err) = self
            .cache_sender
            .send(DispatcherToCacheCmd::PublishPacketSent(
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

    async fn cache_on_session_added(&mut self, listener_id: ListenerId) {
        if let Err(err) = self
            .cache_sender
            .send(DispatcherToCacheCmd::SessionAdded(listener_id, 1))
            .await
        {
            log::error!(
                "Dispatcher: Failed to send SessionAdded cmd, err: {:?}",
                err
            );
        }
    }

    async fn cache_on_session_removed(&mut self, listener_id: ListenerId) {
        if let Err(err) = self
            .cache_sender
            .send(DispatcherToCacheCmd::SessionRemoved(listener_id, 1))
            .await
        {
            log::error!(
                "Dispatcher: Failed to send SessionRemoved cmd, err: {:?}",
                err
            );
        }
    }

    async fn cache_on_subscription_added(&mut self, listener_id: ListenerId) {
        if let Err(err) = self
            .cache_sender
            .send(DispatcherToCacheCmd::SubscriptionsAdded(listener_id, 1))
            .await
        {
            log::error!(
                "Dispatcher: Failed to send SubscriptionsAdded cmd, err: {:?}",
                err
            );
        }
    }
}
