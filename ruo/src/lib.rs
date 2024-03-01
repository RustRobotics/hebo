// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

#![deny(
    warnings,
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic
)]
// TODO(Shaohua): Remove this lint flag
#![allow(clippy::multiple_crate_versions)]

pub mod client;
mod client_inner_v3;
mod client_inner_v5;
pub mod connect_options;
pub mod error;
mod publish;
mod status;
pub mod stream;

#[cfg(feature = "blocking")]
pub mod blocking;

pub use publish::PublishMessage;
pub use status::ClientStatus;

pub(crate) use client_inner_v3::ClientInnerV3;
pub(crate) type ClientInnerV4 = ClientInnerV3;
pub(crate) use client_inner_v5::ClientInnerV5;
