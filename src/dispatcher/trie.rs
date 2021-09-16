// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

//! Manage subscription trie.

use codec::{SubscribeAckPacket, SubscribePacket, SubscribedTopic};
use std::collections::BTreeMap;

use crate::types::SessionGid;

#[derive(Debug, Default, Clone)]
pub struct SubTrie {
    map: BTreeMap<SessionGid, Vec<SubscribedTopic>>,
}

impl SubTrie {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    pub fn subscribe(
        &mut self,
        session_gid: SessionGid,
        packet: SubscribePacket,
    ) -> SubscribeAckPacket {
        unimplemented!()
    }
}
