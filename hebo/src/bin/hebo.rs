// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use hebo::{server, Error};

fn main() -> Result<(), Error> {
    server::run::run_server()
}
