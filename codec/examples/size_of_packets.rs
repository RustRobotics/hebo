// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::mem::size_of;

fn print_basic_types() {
    use hebo_codec::{
        BinaryData, BoolData, ByteArray, PacketId, PubTopic, QoS, StringData, StringPairData,
        U16Data, U32Data, VarInt,
    };
    println!("Binarydata: {}", size_of::<BinaryData>());
    println!("BoolData: {}", size_of::<BoolData>());
    println!("ByteArray: {}", size_of::<ByteArray>());
    println!("PacketId: {}", size_of::<PacketId>());
    println!("PubTopic: {}", size_of::<PubTopic>());
    println!("QoS: {}", size_of::<QoS>());
    println!("StringData: {}", size_of::<StringData>());
    println!("StringPairData: {}", size_of::<StringPairData>());
    println!("U16Data: {}", size_of::<U16Data>());
    println!("U32Data: {}", size_of::<U32Data>());
    println!("VarInt: {}", size_of::<VarInt>());
}

fn print_v3_packets() {
    use hebo_codec::v3::{ConnectAckPacket, ConnectPacket, DisconnectPacket, PingRequestPacket, PingResponsePacket, PublishAckPacket, PublishCompletePacket, PublishPacket, PublishReceivedPacket, PublishReleasePacket, SubscribeAckPacket, SubscribePacket, UnsubscribeAckPacket, UnsubscribePacket};

    println!("==== V3 ====");
    println!("ConnectAckPacket: {}", size_of::<ConnectAckPacket>());
    println!("ConnectPacket: {}", size_of::<ConnectPacket>());
    println!("DisconnectPacket: {}", size_of::<DisconnectPacket>());
    println!("PingRequestPacket: {}", size_of::<PingRequestPacket>());
    println!("PingResponsePacket: {}", size_of::<PingResponsePacket>());
    println!("PublishAckPacket: {}", size_of::<PublishAckPacket>());
    println!(
        "PublishCompletePacket: {}",
        size_of::<PublishCompletePacket>()
    );
    println!("PublishPacket: {}", size_of::<PublishPacket>());
    println!(
        "PublishReceivedPacket: {}",
        size_of::<PublishReceivedPacket>()
    );
    println!(
        "PublishReleasePacket: {}",
        size_of::<PublishReleasePacket>()
    );
    println!("SubscribeAckPacket: {}", size_of::<SubscribeAckPacket>());
    println!("SubscribePacket: {}", size_of::<SubscribePacket>());
    println!(
        "UnsubscribeAckPacket: {}",
        size_of::<UnsubscribeAckPacket>()
    );
    println!("UnsubscribePacket: {}", size_of::<UnsubscribePacket>());
}

fn print_v5_packets() {
    use hebo_codec::v5::{ConnectAckPacket, ConnectPacket, DisconnectPacket, PingRequestPacket, PingResponsePacket, Properties, Property, PublishAckPacket, PublishCompletePacket, PublishPacket, PublishReceivedPacket, PublishReleasePacket, SubscribeAckPacket, SubscribePacket, UnsubscribeAckPacket, UnsubscribePacket};

    println!("==== V5 ====");
    println!("ConnectAckPacket: {}", size_of::<ConnectAckPacket>());
    println!("ConnectPacket: {}", size_of::<ConnectPacket>());
    println!("DisconnectPacket: {}", size_of::<DisconnectPacket>());
    println!("PingRequestPacket: {}", size_of::<PingRequestPacket>());
    println!("PingResponsePacket: {}", size_of::<PingResponsePacket>());
    println!("PublishAckPacket: {}", size_of::<PublishAckPacket>());
    println!(
        "PublishCompletePacket: {}",
        size_of::<PublishCompletePacket>()
    );
    println!("PublishPacket: {}", size_of::<PublishPacket>());
    println!(
        "PublishReceivedPacket: {}",
        size_of::<PublishReceivedPacket>()
    );
    println!(
        "PublishReleasePacket: {}",
        size_of::<PublishReleasePacket>()
    );
    println!("SubscribeAckPacket: {}", size_of::<SubscribeAckPacket>());
    println!("SubscribePacket: {}", size_of::<SubscribePacket>());
    println!(
        "UnsubscribeAckPacket: {}",
        size_of::<UnsubscribeAckPacket>()
    );
    println!("UnsubscribePacket: {}", size_of::<UnsubscribePacket>());

    println!("Property: {}", size_of::<Property>());
    println!("Properties: {}", size_of::<Properties>());
}

fn main() {
    print_basic_types();
    print_v3_packets();
    print_v5_packets();
}
