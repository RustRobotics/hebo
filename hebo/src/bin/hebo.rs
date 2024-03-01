// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use hebo::{server, Error};

fn main() -> Result<(), Error> {
    server::run::handle_cmdline()
}
