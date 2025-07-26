// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

//! Manage subscription trie.

use codec::{v3, v5, SubscribePattern};
use std::collections::HashMap;

use super::Dispatcher;
use crate::commands::DispatcherToListenerCmd;
use crate::types::SessionGid;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Default, Clone)]
pub struct SubTrie {
    map: HashMap<SessionGid, HashMap<String, SubscribePattern>>,
}

impl SubTrie {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn subscribe(
        &mut self,
        session_gid: SessionGid,
        packet: &v3::SubscribePacket,
    ) -> (v3::SubscribeAckPacket, usize) {
        let patterns = if let Some(patterns) = self.map.get_mut(&session_gid) {
            patterns
        } else {
            let patterns = HashMap::new();
            self.map.insert(session_gid, patterns);
            self.map.get_mut(&session_gid).unwrap()
        };

        // If a Server receives a SUBSCRIBE packet that contains multiple Topic Filters
        // it MUST handle that packet as if it had received a sequence of multiple SUBSCRIBE packets,
        // except that it combines their responses into a single SUBACK response [MQTT-3.8.4-4].
        let mut ack_vec = vec![];
        let mut pattern_added = 0;
        for topic in packet.topics() {
            // TODO(Shaohua): Send retained messages.
            // TODO(Shaohua): Check topic filter has been subscribed.
            // TODO(Shaohua): Update qos in SubscribeAck.
            match SubscribePattern::parse(topic.topic(), topic.qos()) {
                Ok(pattern) => {
                    patterns.insert(topic.topic().to_string(), pattern);
                    ack_vec.push(v3::SubscribeAck::QoS(topic.qos()));
                    pattern_added += 1;
                }
                Err(err) => {
                    log::error!(
                        "trie: Invalid subscribe topic: {}, err: {:?}",
                        topic.topic(),
                        err
                    );
                    ack_vec.push(v3::SubscribeAck::Failed);
                }
            }
        }

        (
            v3::SubscribeAckPacket::with_vec(packet.packet_id(), ack_vec),
            pattern_added,
        )
    }

    pub fn subscribe_v5(
        &mut self,
        session_gid: SessionGid,
        packet: &v5::SubscribePacket,
    ) -> (v5::SubscribeAckPacket, usize) {
        let patterns = if let Some(patterns) = self.map.get_mut(&session_gid) {
            patterns
        } else {
            let patterns = HashMap::new();
            self.map.insert(session_gid, patterns);
            self.map.get_mut(&session_gid).unwrap()
        };

        // TODO(Shaohua): Add comments
        let mut reasons = vec![];
        let mut pattern_added = 0;
        for topic in packet.topics() {
            // TODO(Shaohua): Send retained messages.
            // TODO(Shaohua): Check topic filter has been subscribed.
            // TODO(Shaohua): Update qos in SubscribeAck.
            match SubscribePattern::parse(topic.topic(), topic.qos()) {
                Ok(pattern) => {
                    patterns.insert(topic.topic().to_string(), pattern);
                    reasons.push(v5::ReasonCode::Success);
                    pattern_added += 1;
                }
                Err(err) => {
                    log::error!(
                        "trie: Invalid subscribe topic: {}, err: {:?}",
                        topic.topic(),
                        err
                    );
                    reasons.push(v5::ReasonCode::TopicFilterInvalid);
                }
            }
        }

        (
            v5::SubscribeAckPacket::with_vec(packet.packet_id(), reasons),
            pattern_added,
        )
    }

    pub fn unsubscribe(
        &mut self,
        session_gid: SessionGid,
        packet: &v3::UnsubscribePacket,
    ) -> usize {
        self.map.get_mut(&session_gid).map_or_else(
            || {
                log::error!("trie: No subscription for gid: {session_gid:?}");
                0
            },
            |set| {
                let to_be_removed: Vec<String> = packet
                    .topics()
                    .iter()
                    .filter(|topic| set.contains_key(topic.as_ref()))
                    .map(|topic| topic.as_ref().to_string())
                    .collect();
                for p in &to_be_removed {
                    set.remove(p);
                }

                to_be_removed.len()
            },
        )
    }

    pub fn unsubscribe_v5(
        &mut self,
        session_gid: SessionGid,
        packet: &v5::UnsubscribePacket,
    ) -> usize {
        self.map.get_mut(&session_gid).map_or_else(
            || {
                log::error!("trie: No subscription for gid: {session_gid:?}");
                0
            },
            |set| {
                let to_be_removed: Vec<String> = packet
                    .topics()
                    .iter()
                    .filter(|topic| set.contains_key(topic.as_ref()))
                    .map(|topic| topic.as_ref().to_string())
                    .collect();
                for p in &to_be_removed {
                    set.remove(p);
                }

                to_be_removed.len()
            },
        )
    }

    pub fn match_packet(&mut self, packet: &v3::PublishPacket) -> Vec<SessionGid> {
        let mut vec = vec![];
        let topic = packet.topic();
        for (session_gid, topic_patterns) in &self.map {
            for topic_pattern in topic_patterns.values() {
                if topic_pattern.topic().is_match(topic) {
                    vec.push(*session_gid);
                    break;
                }
            }
        }
        vec
    }

    pub fn match_packet_v5(&mut self, packet: &v5::PublishPacket) -> Vec<SessionGid> {
        let mut vec = vec![];
        let topic = packet.topic();
        for (session_gid, topic_patterns) in &self.map {
            for topic_pattern in topic_patterns.values() {
                if topic_pattern.topic().is_match(topic) {
                    vec.push(*session_gid);
                    break;
                }
            }
        }
        vec
    }
}

impl Dispatcher {
    pub(super) async fn publish_packet_to_sub_trie(&mut self, packet: &v3::PublishPacket) {
        // match topic in trie
        for session_gid in self.sub_trie.match_packet(packet) {
            // send packet to listener
            if let Some(listener_sender) = self.listener_senders.get(&session_gid.listener_id()) {
                let cmd =
                    DispatcherToListenerCmd::Publish(session_gid.session_id(), packet.clone());
                if let Err(err) = listener_sender.send(cmd).await {
                    log::error!(
                        "dispatcher: Failed to send publish packet to listener: {}, err: {:?}",
                        session_gid.listener_id(),
                        err
                    );
                }
            } else {
                log::error!(
                    "dispatcher: Failed to get listener sender with id: {}",
                    session_gid.listener_id()
                );
            }
        }
    }

    pub(super) async fn publish_packet_to_sub_trie_v5(&mut self, packet: &v5::PublishPacket) {
        // match topic in trie
        for session_gid in self.sub_trie.match_packet_v5(packet) {
            // send packet to listener
            if let Some(listener_sender) = self.listener_senders.get(&session_gid.listener_id()) {
                let cmd =
                    DispatcherToListenerCmd::PublishV5(session_gid.session_id(), packet.clone());
                if let Err(err) = listener_sender.send(cmd).await {
                    log::error!(
                        "dispatcher: Failed to send publish packet to listener: {}, err: {:?}",
                        session_gid.listener_id(),
                        err
                    );
                }
            } else {
                log::error!(
                    "dispatcher: Failed to get listener sender with id: {}",
                    session_gid.listener_id()
                );
            }
        }
    }
}
