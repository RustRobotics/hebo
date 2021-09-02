// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use clap::Arg;

use hebo::auth::file_auth;
use hebo::error::{Error, ErrorKind};

fn main() -> Result<(), Error> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let matches = clap::App::new("Hebo Passwd")
        .version("0.1.0")
        .author("Xu Shaohua <shaohua@biofan.org>")
        .about("hebo-passwd is a tool for managing password files for hebo")
        .arg(
            Arg::with_name("add")
                .short("a")
                .long("add")
                .takes_value(true)
                .value_name("username:passwd")
                .multiple(true)
                .help("Add username:passwd pair. Or update if username already exists."),
        )
        .arg(
            Arg::with_name("delete")
                .short("d")
                .long("delete")
                .takes_value(true)
                .value_name("username")
                .multiple(true)
                .help("Delete the username rather than adding/updating its password."),
        )
        .arg(
            Arg::with_name("update")
                .short("u")
                .long("update")
                .takes_value(false)
                .help("Update a plain text password file to use hashed passwords"),
        )
        .arg(Arg::with_name("password_file").help("password_file will be crated if not exist"))
        .get_matches();

    let passwd_file = if let Some(file) = matches.value_of("password_file") {
        file
    } else {
        return Err(Error::new(
            ErrorKind::ParameterError,
            "passwordfile is required",
        ));
    };

    if matches.is_present("update") {
        return file_auth::update_file_hash(passwd_file);
    }

    if matches.is_present("delete") {}

    Ok(())
}
