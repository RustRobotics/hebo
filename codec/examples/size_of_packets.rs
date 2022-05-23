// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::mem::size_of;

fn print_v3_packets() {
    use hebo_codec::v3::*;

    println!("==== V3 ====");
    println!("ConnectPacket: {}", size_of::<ConnectPacket>());
    println!("ConnectAckPacket: {}", size_of::<ConnectAckPacket>());
    println!("DisconnectPacket: {}", size_of::<DisconnectPacket>());
    println!("PingRequestPacket: {}", size_of::<PingRequestPacket>());
    println!("PingResponsePacket: {}", size_of::<PingResponsePacket>());
    println!("PublishPacket: {}", size_of::<PublishPacket>());
    println!("PublishAckPacket: {}", size_of::<PublishAckPacket>());
    println!(
        "PublishCompletePacket: {}",
        size_of::<PublishCompletePacket>()
    );
    println!(
        "PublishReceivedPacket: {}",
        size_of::<PublishReceivedPacket>()
    );
    println!(
        "PublishReleasePacket: {}",
        size_of::<PublishReleasePacket>()
    );
    println!("SubscribePacket: {}", size_of::<SubscribePacket>());
    println!("SubscribeAckPacket: {}", size_of::<SubscribeAckPacket>());
    println!("UnsubscribePacket: {}", size_of::<UnsubscribePacket>());
    println!(
        "UnsubscribeAckPacket: {}",
        size_of::<UnsubscribeAckPacket>()
    );
}

fn print_v5_packets() {
    use hebo_codec::v5::*;
    println!("==== V5 ====");
    println!("ConnectPacket: {}", size_of::<ConnectPacket>());
    println!("ConnectAckPacket: {}", size_of::<ConnectAckPacket>());
    println!("DisconnectPacket: {}", size_of::<DisconnectPacket>());
    println!("PingRequestPacket: {}", size_of::<PingRequestPacket>());
    println!("PingResponsePacket: {}", size_of::<PingResponsePacket>());
    println!("PublishPacket: {}", size_of::<PublishPacket>());
    println!("PublishAckPacket: {}", size_of::<PublishAckPacket>());
    println!(
        "PublishCompletePacket: {}",
        size_of::<PublishCompletePacket>()
    );
    println!(
        "PublishReceivedPacket: {}",
        size_of::<PublishReceivedPacket>()
    );
    println!(
        "PublishReleasePacket: {}",
        size_of::<PublishReleasePacket>()
    );
    println!("SubscribePacket: {}", size_of::<SubscribePacket>());
    println!("SubscribeAckPacket: {}", size_of::<SubscribeAckPacket>());
    println!("UnsubscribePacket: {}", size_of::<UnsubscribePacket>());
    println!(
        "UnsubscribeAckPacket: {}",
        size_of::<UnsubscribeAckPacket>()
    );
}

fn main() {
    print_v3_packets();
    print_v5_packets();
}
