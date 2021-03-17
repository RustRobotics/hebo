// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

/// Generate random string.
pub fn random_string(len: usize) -> String {
    String::from_utf8(
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(len)
            .collect::<Vec<u8>>(),
    )
    .unwrap()
}

#[derive(Debug)]
pub enum ClientIdError {
    /// no chars
    IsEmpty,

    /// Larger than 23 chars
    TooLong,

    /// Can only contain 0-9a-zA-Z
    InvalidChars,
}

pub fn check_client_id(_client_id: &str) -> Result<(), ClientIdError> {
    // TODO(Shaohua): Add a regexp
    Ok(())
}
