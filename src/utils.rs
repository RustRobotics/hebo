// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

/// Generate random string.
pub fn random_string(len: usize) -> String {
    thread_rng().sample_iter(&Alphanumeric).take(len).collect()
}
