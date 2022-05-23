// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

//! Metrics service backend.
//! Embed a `sys_tree` module to send $SYS messages to dispatcher.

use codec::{v3, QoS};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time::interval;

use crate::cache_types::{ListenerMetrics, ListenersMapMetrics, SystemMetrics};
use crate::commands::{DispatcherToMetricsCmd, MetricsToDispatcherCmd, ServerContextToMetricsCmd};
use crate::error::Error;
use crate::types::Uptime;

pub const UPTIME: &str = "$SYS/uptime";

/// Key-value store.
#[derive(Debug)]
pub struct Metrics {
    sys_tree_interval: Duration,
    startup: SystemTime,
    uptime: Uptime,

    system: SystemMetrics,
    listeners: ListenersMapMetrics,

    dispatcher_sender: Sender<MetricsToDispatcherCmd>,
    dispatcher_receiver: Receiver<DispatcherToMetricsCmd>,

    server_ctx_receiver: Receiver<ServerContextToMetricsCmd>,
}

impl Metrics {
    #[must_use]
    pub fn new(
        sys_tree_interval: Duration,
        // dispatcher module
        dispatcher_sender: Sender<MetricsToDispatcherCmd>,
        dispatcher_receiver: Receiver<DispatcherToMetricsCmd>,
        // server ctx module
        server_ctx_receiver: Receiver<ServerContextToMetricsCmd>,
    ) -> Self {
        Self {
            sys_tree_interval,
            startup: SystemTime::now(),
            uptime: 0,
            system: SystemMetrics::default(),
            listeners: HashMap::new(),

            dispatcher_sender,
            dispatcher_receiver,

            server_ctx_receiver,
        }
    }

    pub async fn run_loop(&mut self) -> ! {
        // Update uptime property each second.
        let mut sys_tree_uptime_timer = interval(Duration::from_secs(1));
        let mut sys_tree_timer = interval(self.sys_tree_interval);

        loop {
            tokio::select! {
                Some(cmd) = self.dispatcher_receiver.recv() => {
                    self.handle_dispatcher_cmd(cmd).await;
                }

                Some(cmd) = self.server_ctx_receiver.recv() => {
                    self.handle_server_ctx_cmd(cmd).await;
                }

                _ = sys_tree_uptime_timer.tick() => {
                    self.sys_tree_update_uptime();
                }

                _ = sys_tree_timer.tick() => {
                    self.sys_tree_handle_timeout().await;
                }
            }
        }
    }

    #[allow(clippy::too_many_lines)]
    async fn handle_dispatcher_cmd(&mut self, cmd: DispatcherToMetricsCmd) {
        match cmd {
            DispatcherToMetricsCmd::ListenerAdded(listener_id, address) => {
                log::info!("Add listener id: {}, addr: {:?}", listener_id, address);
                assert!(self.listeners.get(&listener_id).is_none());
                let listener_cache = ListenerMetrics::new(listener_id, address);
                self.listeners.insert(listener_id, listener_cache);
                self.system.listener_count += 1;
            }
            DispatcherToMetricsCmd::ListenerRemoved(listener_id) => {
                log::info!("Remove listener with id: {}", listener_id);
                assert!(self.listeners.remove(&listener_id).is_some());
                self.system.listener_count -= 1;
            }
            DispatcherToMetricsCmd::SessionAdded(listener_id, count) => {
                log::info!("{} sessions added to #{}", count, listener_id);
                if let Some(listener) = self.listeners.get_mut(&listener_id) {
                    let count = count as u64;
                    listener.sessions += count;
                    self.system.sessions += count;
                } else {
                    log::error!("Failed to found listener with id: {}", listener_id);
                }
            }
            DispatcherToMetricsCmd::SessionRemoved(listener_id, count) => {
                log::info!("{} sessions removed from #{}", count, listener_id);
                if let Some(listener) = self.listeners.get_mut(&listener_id) {
                    let count = count as u64;
                    listener.sessions -= count;
                    self.system.sessions -= count;
                } else {
                    log::error!("Failed to found listener with id: {}", listener_id);
                }
            }
            DispatcherToMetricsCmd::SubscriptionsAdded(listener_id, count) => {
                log::info!("{} subscriptions added to #{}", count, listener_id);
                if let Some(listener) = self.listeners.get_mut(&listener_id) {
                    let count = count as u64;
                    listener.subscriptions += count;
                    self.system.subscriptions += count;
                } else {
                    log::error!("Failed to found listener with id: {}", listener_id);
                }
            }
            DispatcherToMetricsCmd::SubscriptionsRemoved(listener_id, count) => {
                log::info!("{} subscriptions removed from #{}", count, listener_id);
                if let Some(listener) = self.listeners.get_mut(&listener_id) {
                    let count = count as u64;
                    listener.subscriptions -= count;
                    self.system.subscriptions -= count;
                } else {
                    log::error!("Failed to found listener with id: {}", listener_id);
                }
            }
            DispatcherToMetricsCmd::RetainedMessageAdded(listener_id, count, bytes) => {
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
            DispatcherToMetricsCmd::RetainedMessageRemoved(listener_id, count, bytes) => {
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
            DispatcherToMetricsCmd::PublishPacketSent(listener_id, count, bytes) => {
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
            DispatcherToMetricsCmd::PublishPacketReceived(listener_id, count, bytes) => {
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
            DispatcherToMetricsCmd::PublishPacketDropped(count, bytes) => {
                log::info!("{} publishPacket dropped", count);
                let count = count as u64;
                let bytes = bytes as u64;
                self.system.publish_messages_dropped += count;
                self.system.publish_bytes_dropped += bytes;
            }
            DispatcherToMetricsCmd::PacketSent(listener_id, count, bytes) => {
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
            DispatcherToMetricsCmd::PacketReceived(listener_id, count, bytes) => {
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

    async fn sys_tree_handle_timeout(&mut self) {
        // TODO(Shaohua): Send other messages.
        if let Err(err) = self.sys_tree_send_uptime().await {
            log::error!(
                "Failed to send publish packet from metrics to dispatcher: {:?}",
                err
            );
        }
    }

    fn sys_tree_update_uptime(&mut self) {
        match SystemTime::now().duration_since(self.startup) {
            Ok(duration) => {
                self.uptime = duration.as_secs();
            }
            Err(err) => {
                log::error!("Failed to update uptime, got error: {}", err);
            }
        }
    }

    async fn sys_tree_send_uptime(&mut self) -> Result<(), Error> {
        //log::info!("metrics::sys_tree_send_uptime()");
        let msg = format!("{}", self.uptime).into_bytes();
        let packet = v3::PublishPacket::new(UPTIME, QoS::AtMostOnce, &msg)?;
        self.dispatcher_sender
            .send(MetricsToDispatcherCmd::Publish(packet))
            .await
            .map(drop)
            .map_err(Into::into)
    }

    /// Server context handler
    async fn handle_server_ctx_cmd(&mut self, cmd: ServerContextToMetricsCmd) {
        match cmd {
            ServerContextToMetricsCmd::MetricsGetUptime(resp_tx) => {
                if let Err(err) = resp_tx.send(self.uptime) {
                    log::error!("Failed to send uptime to server ctx: {:?}", err);
                }
            }
        }
    }
}
