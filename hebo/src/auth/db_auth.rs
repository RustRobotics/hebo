// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

//! Interface for auth app database backend.

use crate::error::Error;

pub trait DbAuth {
    /// Check whether (username, password) is matched in database records.
    ///
    /// # Errors
    ///
    /// Returns error if failed to access to database.
    fn is_match(&self, username: &str, password: &[u8]) -> Result<bool, Error>;
}
