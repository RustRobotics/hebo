// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::convert::TryFrom;

use crate::{ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket, QoS};

/// Structure of `ConnectFlags` is:
/// ```txt
///         7               6              5          4-3          2            1             0
/// +---------------+---------------+-------------+----------+-----------+---------------+----------+
/// | Username Flag | Password Flag | Will Retain | Will QoS | Will Flag | Clean Session | Reserved |
/// +---------------+---------------+-------------+----------+-----------+---------------+----------+
/// ```
#[allow(clippy::struct_excessive_bools)]
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectFlags {
    /// `username` field specifies whether `username` shall be presented in the payload.
    has_username: bool,

    /// `password` field specifies whether `password` shall be presented in the payload.
    /// If `username` field is false, then this field shall be false too.
    has_password: bool,

    /// `retain` field specifies if the Will Message is to be retained when it is published.
    /// If the `will` field is false, then the `retain` field msut be false.
    will_retain: bool,

    /// `QoS` level to be used in the Will Message.
    will_qos: QoS,

    /// If this field is set to true, a Will Message will be stored on the Server side when
    /// Client connected, and this message must be sent back when Client connection
    /// is closed abnormally unless it is deleted by the Server on receipt of a Disconnect Packet.
    ///
    /// This Will Message is used mainly to handle errors:
    /// * I/O error or network error
    /// * Keep alive timeout
    /// * network disconnected without Disconnect Packet
    /// * protocol error
    will: bool,

    /// To control how to handle Session State.
    /// If `clean_sessions` is true, the Client and Server must discard any previous Session State
    /// and start a new once until end of Disconnect. So that State data
    /// cannot be reused in subsequent connections.
    ///
    /// Client side of Session State consists of:
    /// * `QoS` 1 and `QoS` 2 messages which have been sent to server but not be acknowledged yet.
    /// * `QoS` 2 messages which have been received from server but have not been fully acknowledged yet.
    ///
    /// Server side of Session State consists of:
    /// * Client subscriptions
    /// * `QoS` 1 and `QoS` 2 messages which have been sent to subscribed Clients,
    ///   but have not been acknowledged yet.
    /// * `QoS` 1 and `QoS` 2 messages pending transmission to the Client.
    /// * `QoS` 2 messages which have been received from the Clients,
    ///   but have not been fully acknowledged yet.
    clean_session: bool,
}

impl ConnectFlags {
    /// Get byte length in packet.
    #[must_use]
    #[inline]
    pub const fn bytes() -> usize {
        1
    }

    /// Update `has_username` flag.
    pub const fn set_has_username(&mut self, has_username: bool) -> &mut Self {
        self.has_username = has_username;
        self
    }

    /// Get current `has_username` flag.
    #[must_use]
    #[inline]
    pub const fn has_username(&self) -> bool {
        self.has_username
    }

    /// Update `has_password` flag.
    pub const fn set_has_password(&mut self, has_password: bool) -> &mut Self {
        self.has_password = has_password;
        self
    }

    /// Get current `has_password` flag.
    #[must_use]
    #[inline]
    pub const fn has_password(&self) -> bool {
        self.has_password
    }

    /// Update will-retain flag.
    pub const fn set_will_retain(&mut self, will_retain: bool) -> &mut Self {
        self.will_retain = will_retain;
        self
    }

    /// Get will-retain flag.
    #[must_use]
    #[inline]
    pub const fn will_retain(&self) -> bool {
        self.will_retain
    }

    /// Update will-qos.
    pub const fn set_will_qos(&mut self, qos: QoS) -> &mut Self {
        self.will_qos = qos;
        self
    }

    /// Get current will-qos.
    #[must_use]
    #[inline]
    pub const fn will_qos(&self) -> QoS {
        self.will_qos
    }

    /// Update will flag.
    pub const fn set_will(&mut self, will: bool) -> &mut Self {
        if !will {
            self.will_qos = QoS::AtMostOnce;
            self.will_retain = false;
        }
        self.will = will;
        self
    }

    /// Get current will flag.
    #[must_use]
    #[inline]
    pub const fn will(&self) -> bool {
        self.will
    }

    /// Update clean-session flag.
    pub const fn set_clean_session(&mut self, clean_session: bool) -> &mut Self {
        self.clean_session = clean_session;
        self
    }

    /// Get clean-session flag.
    #[must_use]
    #[inline]
    pub const fn clean_session(&self) -> bool {
        self.clean_session
    }
}

impl Default for ConnectFlags {
    fn default() -> Self {
        Self {
            has_username: false,
            has_password: false,
            will_retain: false,
            will_qos: QoS::AtMostOnce,
            will: false,
            clean_session: true,
        }
    }
}

impl EncodePacket for ConnectFlags {
    fn encode(&self, v: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let flags = {
            let has_username = if self.has_username {
                0b1000_0000
            } else {
                0b0000_0000
            };
            let has_password = if self.has_password {
                0b0100_0000
            } else {
                0b0000_0000
            };
            let will_retian = if self.will_retain {
                0b0010_0000
            } else {
                0b0000_0000
            };

            let will_qos = match self.will_qos {
                QoS::AtMostOnce => 0b0000_0000,
                QoS::AtLeastOnce => 0b0000_1000,
                QoS::ExactOnce => 0b0001_0000,
            };

            let will = if self.will { 0b0000_0100 } else { 0b0000_0000 };

            let clean_session = if self.clean_session {
                0b0000_0010
            } else {
                0b0000_0000
            };

            has_username | has_password | will_retian | will_qos | will | clean_session
        };
        v.push(flags);

        Ok(1)
    }
}

impl DecodePacket for ConnectFlags {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let flags = ba.read_byte()?;
        let has_username = flags & 0b1000_0000 == 0b1000_0000;
        let has_password = flags & 0b0100_0000 == 0b0100_0000;
        let will_retain = flags & 0b0010_0000 == 0b0010_0000;
        let will_qos = QoS::try_from((flags & 0b0001_1000) >> 3)?;
        let will = flags & 0b0000_0100 == 0b0000_0100;
        let clean_session = flags & 0b0000_0010 == 0b0000_0010;

        // The Server MUST validate that the reserved flag in the CONNECT Control Packet
        // is set to zero and disconnect the Client if it is not zero [MQTT-3.1.2-3].
        let reserved_is_zero = flags & 0b0000_0001 == 0b0000_0000;
        if !reserved_is_zero {
            return Err(DecodeError::InvalidConnectFlags);
        }

        // If the User Name Flag is set to 0, the Password Flag MUST be set to 0. [MQTT-3.1.2-22]
        if !has_username && has_password {
            return Err(DecodeError::InvalidConnectFlags);
        }

        Ok(Self {
            has_username,
            has_password,
            will_retain,
            will_qos,
            will,
            clean_session,
        })
    }
}
