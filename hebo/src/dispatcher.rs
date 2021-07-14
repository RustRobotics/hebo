// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use codec::PublishPacket;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::commands::{
    CacheToDispatcherCmd, DispatcherToCacheCmd, DispatcherToListenerCmd, ListenerToDispatcherCmd,
    SystemToDispatcherCmd,
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
                let (send_ok, send_failed) = self.publish_packet_to_listners(&packet).await;
                self.cache_update_publish_packet(42, send_ok, send_failed)
                    .await;
            }
        }
    }

    async fn handle_listener_cmd(&mut self, cmd: ListenerToDispatcherCmd) {
        log::info!("handle_listener_cmd: {:?}", cmd);
        match cmd {
            ListenerToDispatcherCmd::Publish(packet) => {
                self.storage_store_packet(&packet).await;
                let (send_ok, send_failed) = self.publish_packet_to_listners(&packet).await;
                self.cache_update_publish_packet(42, send_ok, send_failed)
                    .await;
            }
        }
    }

    /// Send packet to storage.
    async fn storage_store_packet(&mut self, packet: &PublishPacket) {
        log::info!("store packet: {:?}", packet);
    }

    async fn publish_packet_to_listners(&mut self, packet: &PublishPacket) -> (usize, usize) {
        let mut send_ok = 0;
        let mut send_failed = 0;
        for (_listener_id, sender) in &self.listener_senders {
            let cmd = DispatcherToListenerCmd::Publish(packet.clone());
            if let Err(err) = sender.send(cmd).await {
                log::error!("Dispatcher::handle_listener_cmd() send failed: {:?}", err);
                send_failed += 1;
            } else {
                send_ok += 1;
            }
        }
        (send_ok, send_failed)
    }

    //
    // Cache related methods
    //
    async fn cache_update_publish_packet(
        &mut self,
        packet_size: usize,
        send_ok: usize,
        send_failed: usize,
    ) {
        log::info!(
            "update publish packet, packet size: {}, send oK: {}, send failed: {}",
            packet_size,
            send_ok,
            send_failed
        );
    }
}
