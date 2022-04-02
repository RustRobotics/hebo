// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use std::collections::HashMap;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::commands::{
    BackendsToDispatcherCmd, BridgeToDispatcherCmd, DispatcherToBackendsCmd, DispatcherToBridgeCmd,
    DispatcherToGatewayCmd, DispatcherToListenerCmd, DispatcherToMetricsCmd,
    DispatcherToRuleEngineCmd, GatewayToDispatcherCmd, ListenerToDispatcherCmd,
    MetricsToDispatcherCmd, RuleEngineToDispatcherCmd,
};
use crate::types::ListenerId;

mod backends;
mod bridge;
mod gateway;
mod listener;
mod metrics;
mod rule_engine;
mod sessions;
mod trie;

/// Dispatcher is a message router.
#[derive(Debug)]
pub struct Dispatcher {
    sub_trie: trie::SubTrie,

    cached_sessions: sessions::CachedSessions,

    backends_sender: Sender<DispatcherToBackendsCmd>,
    backends_receiver: Receiver<BackendsToDispatcherCmd>,

    bridge_sender: Sender<DispatcherToBridgeCmd>,
    bridge_receiver: Receiver<BridgeToDispatcherCmd>,

    gateway_sender: Sender<DispatcherToGatewayCmd>,
    gateway_receiver: Receiver<GatewayToDispatcherCmd>,

    metrics_sender: Sender<DispatcherToMetricsCmd>,
    metrics_receiver: Receiver<MetricsToDispatcherCmd>,

    listener_senders: HashMap<ListenerId, Sender<DispatcherToListenerCmd>>,
    listener_receiver: Receiver<ListenerToDispatcherCmd>,

    rule_engine_sender: Sender<DispatcherToRuleEngineCmd>,
    rule_engine_receiver: Receiver<RuleEngineToDispatcherCmd>,
}

impl Dispatcher {
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
        Self {
            sub_trie: trie::SubTrie::new(),

            cached_sessions: sessions::CachedSessions::new(),

            backends_sender,
            backends_receiver,

            bridge_sender,
            bridge_receiver,

            gateway_sender,
            gateway_receiver,

            metrics_sender,
            metrics_receiver,

            listener_senders: listener_senders.into_iter().collect(),
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
}
