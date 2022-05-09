// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

pub mod client;
pub mod connect_options;
pub mod error;
mod publish;
mod status;
pub mod stream;

#[cfg(feature = "blocking")]
pub mod blocking;

pub use publish::PublishMessage;
pub use status::ClientStatus;

mod client_inner_v3;
mod client_inner_v5;
pub(crate) use client_inner_v3::ClientInnerV3;
pub(crate) type ClientInnerV4 = ClientInnerV3;
pub(crate) use client_inner_v5::ClientInnerV5;
