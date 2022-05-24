// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::{DecodeError, U16Data};

/// KeepAlive presents connection keep-alive time in milliseconds.
pub type KeepAlive = U16Data;

/// Check `keep_alive` is in range.
///
/// # Errors
///
/// Returns error if `keep_alive` value is too small.
pub const fn validate_keep_alive(keep_alive: KeepAlive) -> Result<(), DecodeError> {
    if keep_alive.value() != 0 && keep_alive.value() < 5 {
        Err(DecodeError::OtherErrors)
    } else {
        Ok(())
    }
}
