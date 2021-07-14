// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::collections::HashMap;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::commands::{CacheToDispatcherCmd, DispatcherToCacheCmd};
use crate::error::Error;

/// Key-value store.
#[derive(Debug)]
pub struct Cache {
    sender: Sender<CacheToDispatcherCmd>,
    receiver: Option<Receiver<DispatcherToCacheCmd>>,

    system: SystemCache,

    listeners: HashMap<u32, ListenerCache>,
}

#[derive(Debug, Default)]
pub struct ListenerCache {
    pub id: u32,
    pub address: String,

    pub sessions: u64,

    pub subscriptions: u64,

    pub retained_messages: u64,
    pub retained_bytes: u64,

    pub messages_sent: u64,
    pub messages_received: u64,

    pub bytes_sent: u64,
    pub bytes_received: u64,

    pub publish_messages_sent: u64,
    pub publish_messages_received: u64,

    pub publish_bytes_sent: u64,
    pub publish_bytes_received: u64,
}

impl ListenerCache {
    fn new(id: u32, address: String) -> Self {
        ListenerCache {
            id,
            address,
            ..Self::default()
        }
    }
}

#[derive(Debug, Default)]
pub struct SystemCache {
    pub listener_count: usize,
    pub sessions: u64,
    pub subscriptions: u64,

    pub retained_messages: u64,
    pub retained_bytes: u64,

    pub messages_sent: u64,
    pub messages_received: u64,

    pub bytes_sent: u64,
    pub bytes_received: u64,

    pub publish_messages_dropped: u64,
    pub publish_messages_sent: u64,
    pub publish_messages_received: u64,

    pub publish_bytes_dropped: u64,
    pub publish_bytes_sent: u64,
    pub publish_bytes_received: u64,
}

impl Cache {
    pub fn new(
        sender: Sender<CacheToDispatcherCmd>,
        receiver: Receiver<DispatcherToCacheCmd>,
    ) -> Self {
        Cache {
            sender,
            receiver: Some(receiver),
            system: SystemCache::default(),
            listeners: HashMap::new(),
        }
    }

    pub async fn run_loop(&mut self) -> ! {
        let mut dispatcher_receiver = self.receiver.take().expect("Invalid dispatcher receiver");
        loop {
            if let Some(cmd) = dispatcher_receiver.recv().await {
                self.handle_dispatcher_cmd(cmd).await;
            }
        }
    }

    async fn handle_dispatcher_cmd(&mut self, cmd: DispatcherToCacheCmd) {
        log::info!("cmd: {:?}", cmd);
        match cmd {
            DispatcherToCacheCmd::ListenerAdded(listener_id, address) => {
                log::info!("Add listener id: {}, addr: {:?}", listener_id, address);
                assert!(self.listeners.get(&listener_id).is_none());
                let listener_cache = ListenerCache::new(listener_id, address);
                self.listeners.insert(listener_id, listener_cache);
                self.system.listener_count += 1;
            }
            DispatcherToCacheCmd::ListenerRemoved(listener_id) => {
                log::info!("Remove listener with id: {}", listener_id);
                assert!(self.listeners.remove(&listener_id).is_some());
                self.system.listener_count -= 1;
            }
            DispatcherToCacheCmd::SessionAdded(listener_id, count) => {
                log::info!("{} sessions added to #{}", count, listener_id);
                if let Some(listener) = self.listeners.get_mut(&listener_id) {
                    let count = count as u64;
                    listener.sessions += count;
                    self.system.sessions += count;
                } else {
                    log::error!("Failed to found listener with id: {}", listener_id);
                }
            }
            DispatcherToCacheCmd::SessionRemoved(listener_id, count) => {
                log::info!("{} sessions removed from #{}", count, listener_id);
                if let Some(listener) = self.listeners.get_mut(&listener_id) {
                    let count = count as u64;
                    listener.sessions -= count;
                    self.system.sessions -= count;
                } else {
                    log::error!("Failed to found listener with id: {}", listener_id);
                }
            }
            DispatcherToCacheCmd::SubscriptionsAdded(listener_id, count) => {
                log::info!("{} subscriptions added to #{}", count, listener_id);
                if let Some(listener) = self.listeners.get_mut(&listener_id) {
                    let count = count as u64;
                    listener.subscriptions += count;
                    self.system.subscriptions += count;
                } else {
                    log::error!("Failed to found listener with id: {}", listener_id);
                }
            }
            DispatcherToCacheCmd::SubscriptionsRemoved(listener_id, count) => {
                log::info!("{} subscriptions removed from #{}", count, listener_id);
                if let Some(listener) = self.listeners.get_mut(&listener_id) {
                    let count = count as u64;
                    listener.subscriptions -= count;
                    self.system.subscriptions -= count;
                } else {
                    log::error!("Failed to found listener with id: {}", listener_id);
                }
            }
            DispatcherToCacheCmd::RetainedMessageAdded(listener_id, count, bytes) => {
                log::info!("{} retained messages added to #{}", count, listener_id);
                if let Some(listener) = self.listeners.get_mut(&listener_id) {
                    let count = count as u64;
                    let bytes = bytes as u64;
                    listener.retained_messages += count;
                    listener.retained_bytes += bytes;
                    self.system.retained_messages += count;
                    self.system.retained_bytes += bytes;
                } else {
                    log::error!("Failed to found listener with id: {}", listener_id);
                }
            }
            DispatcherToCacheCmd::RetainedMessageRemoved(listener_id, count, bytes) => {
                log::info!("{} retained messages removed from #{}", count, listener_id);
                if let Some(listener) = self.listeners.get_mut(&listener_id) {
                    let count = count as u64;
                    let bytes = bytes as u64;
                    listener.retained_messages -= count;
                    listener.retained_bytes -= bytes;
                    self.system.retained_messages -= count;
                    self.system.retained_bytes -= bytes;
                } else {
                    log::error!("Failed to found listener with id: {}", listener_id);
                }
            }
            DispatcherToCacheCmd::PublishPacketSent(listener_id, count, bytes) => {
                log::info!("{} publishPacketSent added to #{}", count, listener_id);
                if let Some(listener) = self.listeners.get_mut(&listener_id) {
                    let count = count as u64;
                    let bytes = bytes as u64;
                    listener.publish_messages_sent += count;
                    listener.publish_bytes_sent += bytes;
                    self.system.publish_messages_sent += count;
                    self.system.publish_bytes_sent += bytes;
                } else {
                    log::error!("Failed to found listener with id: {}", listener_id);
                }
            }
            DispatcherToCacheCmd::PublishPacketReceived(listener_id, count, bytes) => {
                log::info!("{} publishPacketReceived added to #{}", count, listener_id);
                if let Some(listener) = self.listeners.get_mut(&listener_id) {
                    let count = count as u64;
                    let bytes = bytes as u64;
                    listener.publish_messages_received += count;
                    listener.publish_bytes_received += bytes;
                    self.system.publish_messages_received += count;
                    self.system.publish_bytes_received += bytes;
                } else {
                    log::error!("Failed to found listener with id: {}", listener_id);
                }
            }
            DispatcherToCacheCmd::PublishPacketDropped(count, bytes) => {
                log::info!("{} publishPacket dropped", count);
                let count = count as u64;
                let bytes = bytes as u64;
                self.system.publish_messages_dropped += count;
                self.system.publish_bytes_dropped += bytes;
            }
            DispatcherToCacheCmd::PacketSent(listener_id, count, bytes) => {
                log::info!("{} packetSent added to #{}", count, listener_id);
                if let Some(listener) = self.listeners.get_mut(&listener_id) {
                    let count = count as u64;
                    let bytes = bytes as u64;
                    listener.messages_sent += count;
                    listener.bytes_sent += bytes;
                    self.system.messages_sent += count;
                    self.system.bytes_sent += bytes;
                } else {
                    log::error!("Failed to found listener with id: {}", listener_id);
                }
            }
            DispatcherToCacheCmd::PacketReceived(listener_id, count, bytes) => {
                log::info!("{} packetReceived added to #{}", count, listener_id);
                if let Some(listener) = self.listeners.get_mut(&listener_id) {
                    let count = count as u64;
                    let bytes = bytes as u64;
                    listener.messages_received += count;
                    listener.bytes_received += bytes;
                    self.system.messages_received += count;
                    self.system.bytes_received += bytes;
                } else {
                    log::error!("Failed to found listener with id: {}", listener_id);
                }
            }

            _ => {
                // Do nothing
            }
        }
    }
}
