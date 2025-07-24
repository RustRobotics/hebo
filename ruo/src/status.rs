// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

/// Mqtt connection status.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClientStatus {
    /// The client sends a Connect packet to the server and waits for a reply packet.
    Connecting,

    /// The client is connected to the server.
    /// Publish/subscribe packets can be sent now.
    Connected,

    /// The client prepares to send Disconnect packets from the server.
    /// No other packets shall be send any more.
    Disconnecting,

    /// The client is disconnected from the server.
    Disconnected,
}
