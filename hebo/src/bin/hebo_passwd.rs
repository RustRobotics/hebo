// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use clap::Arg;

use hebo::auth::file_auth;
use hebo::error::{Error, ErrorKind};

fn main() -> Result<(), Error> {
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
        .arg(
            Arg::with_name("password_file")
                .required(true)
                .help("password_file will be crated if not exist"),
        )
        .get_matches();

    let passwd_file = if let Some(file) = matches.value_of("password_file") {
        file
    } else {
        return Err(Error::new(
            ErrorKind::ParameterError,
            "password_file is required",
        ));
    };

    if matches.is_present("update") {
        return file_auth::update_file_hash(passwd_file);
    }

    let add_users: Vec<&str> = if let Some(users) = matches.values_of("add") {
        users.collect()
    } else {
        Vec::new()
    };

    let delete_users: Vec<&str> = if let Some(users) = matches.values_of("delete") {
        users.collect()
    } else {
        Vec::new()
    };

    file_auth::add_delete_users(passwd_file, &add_users, &delete_users)?;

    Ok(())
}
