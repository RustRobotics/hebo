// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

/// The Client uses this value to limit the number of QoS 1 and QoS 2 publications that
/// it is willing to process concurrently. There is no mechanism to limit
/// the QoS 0 publications that the Server might try to send.
pub const DEFAULT_RECEIVE_MAXIMUM: u16 = u16::MAX;
