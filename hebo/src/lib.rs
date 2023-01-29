// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
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

pub mod auth;
pub mod backends;
pub mod bridge;
pub mod cache_types;
pub mod commands;
pub mod config;
pub mod connectors;
pub mod dashboard;
pub mod dispatcher;
pub mod error;
pub mod gateway;
pub mod listener;
pub mod log;
pub mod metrics;
pub mod server;
pub mod session;
pub mod socket;
pub mod stream;
pub mod types;


#[cfg(feature = "acl")]
pub mod acl;

pub mod rule_engine;

pub use error::Error;
