// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

//! Manage subscription trie.

use codec::{
    v3::{PublishPacket, SubscribeAck, SubscribeAckPacket, SubscribePacket, UnsubscribePacket},
    SubscribePattern,
};
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
        packet: &SubscribePacket,
    ) -> (SubscribeAckPacket, usize) {
        let patterns = match self.map.get_mut(&session_gid) {
            Some(patterns) => patterns,
            None => {
                let patterns = HashMap::new();
                self.map.insert(session_gid, patterns);
                self.map.get_mut(&session_gid).unwrap()
            }
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
                    ack_vec.push(SubscribeAck::QoS(topic.qos()));
                    pattern_added += 1;
                }
                Err(err) => {
                    log::error!(
                        "trie: Invalid subscribe topic: {}, err: {:?}",
                        topic.topic(),
                        err
                    );
                    ack_vec.push(SubscribeAck::Failed);
                }
            }
        }

        (
            SubscribeAckPacket::with_vec(packet.packet_id(), ack_vec),
            pattern_added,
        )
    }

    pub fn unsubscribe(&mut self, session_gid: SessionGid, packet: &UnsubscribePacket) -> usize {
        self.map.get_mut(&session_gid).map_or_else(
            || {
                log::error!("trie: No subscription for gid: {:?}", session_gid);
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

    pub fn match_packet(&mut self, packet: &PublishPacket) -> Vec<SessionGid> {
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
    pub(super) async fn publish_packet_to_sub_trie(&mut self, packet: &PublishPacket) {
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
}
