// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::collections::HashMap;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::cache_types::{ListenerCache, ListenersMapCache, ListenersVectorCache, SystemCache};
use crate::commands::{
    CacheToDispatcherCmd, CacheToSystemCmd, DispatcherToCacheCmd, SystemToCacheCmd,
};

/// Key-value store.
#[derive(Debug)]
pub struct Cache {
    dispatcher_sender: Sender<CacheToDispatcherCmd>,
    dispatcher_receiver: Option<Receiver<DispatcherToCacheCmd>>,

    system_sender: Sender<CacheToSystemCmd>,
    system_receiver: Option<Receiver<SystemToCacheCmd>>,

    system: SystemCache,

    listeners: ListenersMapCache,
}

impl Cache {
    pub fn new(
        dispatcher_sender: Sender<CacheToDispatcherCmd>,
        dispatcher_receiver: Receiver<DispatcherToCacheCmd>,
        system_sender: Sender<CacheToSystemCmd>,
        system_receiver: Receiver<SystemToCacheCmd>,
    ) -> Self {
        Cache {
            dispatcher_sender,
            dispatcher_receiver: Some(dispatcher_receiver),
            system_sender,
            system_receiver: Some(system_receiver),
            system: SystemCache::default(),
            listeners: HashMap::new(),
        }
    }

    pub async fn run_loop(&mut self) -> ! {
        let mut dispatcher_receiver = self
            .dispatcher_receiver
            .take()
            .expect("Invalid dispatcher receiver");
        let mut system_receiver = self
            .system_receiver
            .take()
            .expect("Invalid system receiver");
        loop {
            tokio::select! {
                Some(cmd) = dispatcher_receiver.recv() => {
                    self.handle_dispatcher_cmd(cmd).await;
                }
                Some(cmd) = system_receiver.recv() => {
                    self.handle_system_cmd(cmd).await;
                }
            }
        }
    }

    async fn handle_dispatcher_cmd(&mut self, cmd: DispatcherToCacheCmd) {
        log::info!("cmd: {:?}", cmd);
        match cmd {
            DispatcherToCacheCmd::ListenerAdded(listener_id, address) => {
                log::info!("Add listener id: {}, addr: {:?}", listener_id, address);
                assert!(self.listeners.get(&listener_id).is_none());
                let listener_cache = ListenerCache::new(listener_id, address.to_string());
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
        }
    }

    async fn handle_system_cmd(&mut self, cmd: SystemToCacheCmd) {
        log::info!("cmd: {:?}", cmd);
        match cmd {
            SystemToCacheCmd::GetAllCache => {
                let v = self.listeners.values().map(|v| v.clone()).collect();
                if let Err(err) = self
                    .system_sender
                    .send(CacheToSystemCmd::All(self.system, v))
                    .await
                {
                    log::error!("Failed to send All cache cmd: {:?}", err);
                }
            }
            SystemToCacheCmd::GetSystemCache => {
                if let Err(err) = self
                    .system_sender
                    .send(CacheToSystemCmd::System(self.system))
                    .await
                {
                    log::error!("Failed to send System cmd: {:?}", err);
                }
            }
            SystemToCacheCmd::GetListenersCache => {
                let v = self.listeners.values().map(|v| v.clone()).collect();
                if let Err(err) = self
                    .system_sender
                    .send(CacheToSystemCmd::Listeners(v))
                    .await
                {
                    log::error!("Failed to send Listeners cmd: {:?}", err);
                }
            }
        }
    }
}
