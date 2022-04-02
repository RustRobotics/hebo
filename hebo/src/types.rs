// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use codec::QoS;

pub type ListenerId = u32;
pub type SessionId = u64;
pub type Uptime = u64;

/// Global session id.
///
/// Basically it is a (listener_id, session_id) pair.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SessionGid {
    listener_id: ListenerId,
    session_id: SessionId,
}

impl SessionGid {
    pub fn new(listener_id: ListenerId, session_id: SessionId) -> Self {
        Self {
            listener_id,
            session_id,
        }
    }

    pub fn listener_id(&self) -> ListenerId {
        self.listener_id
    }

    pub fn session_id(&self) -> SessionId {
        self.session_id
    }
}

/// Represents a session object.
#[derive(Debug, Clone)]
pub struct SessionInfo {
    pub listener_id: ListenerId,
    pub session_id: SessionId,
    pub qos: QoS,
    pub client_id: String,
    pub ip: String,
    pub connected_at: u64,
    pub tls: bool,
}
