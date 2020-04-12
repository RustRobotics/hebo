// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#[derive(Debug)]
pub enum ServerCommand {
    PublishPacket,
    Subscribe,
}

#[derive(Debug)]
pub enum ConnectionCommand {
    PublishPacket,
    Subscribe,
    Unsubscribe,
}