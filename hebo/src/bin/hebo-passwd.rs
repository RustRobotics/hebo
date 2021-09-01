// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use clap::Arg;

fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let matches = clap::App::new("Hebo Passwd")
        .version("0.1.0")
        .author("Xu Shaohua <shaohua@biofan.org>")
        .about("hebo-passwd is a tool for managing password files for hebo")
        .arg(
            Arg::with_name("batch")
                .short("b")
                .long("batch")
                .takes_value(false)
                .help("run in batch mode to allow passing passwords on the command line."),
        )
        .arg(
            Arg::with_name("delete")
                .short("d")
                .long("delete")
                .takes_value(false)
                .help("delete the username rather than adding/updating its password."),
        )
        .arg(
            Arg::with_name("update")
                .short("U")
                .long("update")
                .takes_value(false)
                .help("Update a plain text password file to use hashed passwords"),
        )
        .arg(Arg::with_name("passwordfile").help("passwordfile will be crated if not exist"))
        .arg(Arg::with_name("username"))
        .arg(Arg::with_name("password"))
        .get_matches();
    log::info!("matches: {:?}", matches);
}
