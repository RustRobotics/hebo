// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

//! Manage subscription trie.

use codec::{PublishPacket, SubscribeAck, SubscribeAckPacket, SubscribePacket, SubscribePattern};
use std::collections::{BTreeSet, HashMap};

use super::Dispatcher;
use crate::commands::DispatcherToListenerCmd;
use crate::types::SessionGid;

#[derive(Debug, Default, Clone)]
pub struct SubTrie {
    map: HashMap<SessionGid, BTreeSet<SubscribePattern>>,
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
        packet: SubscribePacket,
    ) -> SubscribeAckPacket {
        let patterns = match self.map.get_mut(&session_gid) {
            Some(patterns) => patterns,
            None => {
                let patterns = BTreeSet::new();
                self.map.insert(session_gid, patterns);
                self.map.get_mut(&session_gid).unwrap()
            }
        };
        let mut ack_vec = vec![];
        for topic in packet.topics() {
            match SubscribePattern::parse(topic.topic(), topic.qos()) {
                Ok(pattern) => {
                    patterns.insert(pattern);
                    ack_vec.push(SubscribeAck::QoS(topic.qos()));
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

        SubscribeAckPacket::with_vec(ack_vec, packet.packet_id())
    }

    pub fn match_packet(&mut self, packet: &PublishPacket) -> Vec<SessionGid> {
        let mut vec = vec![];
        let topic = packet.topic();
        for (session_gid, topic_patterns) in self.map.iter() {
            for topic_pattern in topic_patterns {
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
