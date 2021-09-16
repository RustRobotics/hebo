// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use codec::SubscribedTopic;
use tokio::sync::mpsc::Sender;

use crate::commands::ListenerToSessionCmd;
use crate::types::SessionId;

#[derive(Debug)]
pub struct Pipeline {
    pub(super) sender: Sender<ListenerToSessionCmd>,
    pub(super) topics: Vec<SubscribedTopic>,
    pub(super) session_id: SessionId,
}

impl Pipeline {
    pub fn new(sender: Sender<ListenerToSessionCmd>, session_id: SessionId) -> Pipeline {
        Pipeline {
            sender,
            topics: Vec::new(),
            session_id,
        }
    }
}
