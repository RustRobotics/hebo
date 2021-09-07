// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use codec::QoS;

pub type ListenerId = u32;
pub type SessionId = u64;

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
