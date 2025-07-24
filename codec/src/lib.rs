// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#![deny(
    warnings,
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic
)]

pub mod base;
mod binary_data;
mod bool_data;
pub mod byte_array;
mod connect_flags;
pub mod error;
mod header;
mod keep_alive;
mod protocol_level;
mod string_data;
mod string_pair_data;
pub mod topic;
mod u16_data;
mod u32_data;
pub mod utils;
mod var_int;

pub use base::{DecodePacket, EncodePacket, QoS};
pub use binary_data::BinaryData;
pub use bool_data::BoolData;
pub use byte_array::ByteArray;
pub use error::{DecodeError, EncodeError};
pub use header::{FixedHeader, Packet, PacketType};
pub use keep_alive::{validate_keep_alive, KeepAlive};
pub use protocol_level::ProtocolLevel;
pub use string_data::StringData;
pub use string_pair_data::StringPairData;
pub use topic::{PubTopic, SubTopic, SubscribePattern, Topic, TopicError, TopicPart};
pub use u16_data::U16Data;
pub use u32_data::U32Data;
pub use var_int::{VarInt, VarIntError};

pub mod v3;
pub mod v5;

/// Packet identifier
pub type PacketId = U16Data;
