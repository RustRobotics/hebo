// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

//! Blocking client is used for testing only.

pub mod client;
mod stream;
pub use stream::Stream;

mod client_inner_v3;
mod client_inner_v5;
use client_inner_v3::ClientInnerV3;
type ClientInnerV4 = ClientInnerV3;
use client_inner_v5::ClientInnerV5;
