// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

use codec::QoS;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone)]
pub struct PublishMessage {
    pub topic: String,
    pub qos: QoS,
    pub payload: Vec<u8>,
}
