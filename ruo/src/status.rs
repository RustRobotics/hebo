// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

/// Mqtt connection status.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ClientStatus {
    Initialized,
    Connecting,
    Connected,
    ConnectFailed,
    Disconnecting,
    Disconnected,
}
