// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser Apache-2.0 License that can be found
// in the LICENSE file.

use codec::v5;

use super::Session;

impl Session {
    /// Handle properties in connect packet.
    pub(super) fn process_connect_properties(&mut self, packet: &v5::ConnectPacket) {
        for property in packet.properties().as_ref() {
            match property {
                v5::Property::SessionExpiryInterval(interval) => {
                    self.config.set_session_expiry_interval(interval.value());
                }
                v5::Property::ReceiveMaximum(receive) => {
                    // TODO(Shaohua): Check receive > 0
                    self.config.set_maximum_inflight_messages(receive.value());
                }
                v5::Property::MaximumPacketSize(packet_size) => {
                    self.config.set_maximum_packet_size(packet_size.value());
                }
                v5::Property::TopicAliasMaximum(topic_alias) => {
                    self.config.set_maximum_topic_alias(topic_alias.value());
                }
                _ => {
                    // todo!()
                }
            }
        }
    }
}
