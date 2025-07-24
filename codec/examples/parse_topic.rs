// Copyright (c) 2025 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use hebo_codec::topic::Topic;

fn main() {
    let t_sys = Topic::parse("$SYS/dev/cpu/+").unwrap();
    println!("t_sys: {t_sys:?}");
    assert!(t_sys.is_match("$SYS/dev/cpu/01"));
}
