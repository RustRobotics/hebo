// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

//! Manage subscription trie.

use codec::{QoS, SubscribeAck, SubscribeAckPacket, SubscribePacket, SubscribePattern};
use std::collections::{BTreeSet, HashMap};

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
        let mut patterns = match self.map.get_mut(&session_gid) {
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
                    log::error!("trie: Invalid subscribe topic: {}", topic.topic());
                    ack_vec.push(SubscribeAck::Failed);
                }
            }
        }

        SubscribeAckPacket::with_vec(ack_vec, packet.packet_id())
    }

    // TODO(Shaohua): Add router()
    #[allow(dead_code)]
    fn topic_match(topics: &[SubscribePattern], topic_str: &str) -> bool {
        for topic in topics {
            if topic.topic().is_match(topic_str) {
                return true;
            }
        }
        false
    }
}
