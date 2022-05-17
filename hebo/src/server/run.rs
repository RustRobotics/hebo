// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use clap::{Arg, ArgMatches, Command};
use std::iter::Iterator;
use std::path::Path;
use tokio::runtime::Runtime;

use super::ServerContext;
use crate::auth::file_auth;
use crate::config::Config;
use crate::error::{Error, ErrorKind};
use crate::log::init_log;

pub const DEFAULT_CONFIG: &str = "/etc/hebo/hebo.toml";
const OPT_CONFIG: &str = "config";
const OPT_RELOAD: &str = "reload";
const OPT_STOP: &str = "stop";
const OPT_TEST: &str = "test";
const SUBCMD_PASSWORD: &str = "password";
const OPT_ADD: &str = "add";
const OPT_UPDATE: &str = "update";
const OPT_DELETE: &str = "delete";
const OPT_PASSWORD_FILE: &str = "password_file";

fn get_cmdline() -> Command<'static> {
    Command::new("Hebo")
        .version("0.2.5")
        .author("Xu Shaohua <shaohua@biofan.org>")
        .about("High Performance MQTT Server")
        .subcommand(
            Command::new(SUBCMD_PASSWORD)
                .about("Manages password files for hebo")
                .arg(
                    Arg::new(OPT_ADD)
                        .short('a')
                        .long(OPT_ADD)
                        .takes_value(true)
                        .value_name("username:passwd")
                        .multiple_occurrences(true)
                        .help("Add username:passwd pair. Or update if username already exists."),
                )
                .arg(
                    Arg::new(OPT_DELETE)
                        .short('d')
                        .long(OPT_DELETE)
                        .takes_value(true)
                        .value_name("username")
                        .multiple_occurrences(true)
                        .help("Delete the username rather than adding/updating its password."),
                )
                .arg(
                    Arg::new(OPT_UPDATE)
                        .short('u')
                        .long(OPT_UPDATE)
                        .takes_value(false)
                        .help("Update a plain text password file to use hashed passwords"),
                )
                .arg(
                    Arg::new(OPT_PASSWORD_FILE)
                        .required(true)
                        .help("password_file will be crated if not exist"),
                ),
        )
        .arg(
            Arg::new(OPT_CONFIG)
                .short('c')
                .long(OPT_CONFIG)
                .value_name("config_file")
                .takes_value(true)
                .help("Specify config file path"),
        )
        .arg(
            Arg::new(OPT_RELOAD)
                .short('r')
                .long(OPT_RELOAD)
                .takes_value(false)
                .help("Reload config"),
        )
        .arg(
            Arg::new(OPT_STOP)
                .short('s')
                .long(OPT_STOP)
                .takes_value(false)
                .help("Stop server"),
        )
        .arg(
            Arg::new(OPT_TEST)
                .short('t')
                .long(OPT_TEST)
                .takes_value(false)
                .help("Test config file and exit"),
        )
}

fn handle_password_subcmd(matches: &ArgMatches) -> Result<(), Error> {
    let password_file = if let Some(file) = matches.value_of(OPT_PASSWORD_FILE) {
        file
    } else {
        return Err(Error::new(
            ErrorKind::ParameterError,
            "password_file is required",
        ));
    };

    if matches.is_present(OPT_UPDATE) {
        return file_auth::update_file_hash(password_file);
    }

    let add_users = matches
        .values_of(OPT_ADD)
        .map_or_else(Vec::new, Iterator::collect);
    let delete_users = matches
        .values_of(OPT_DELETE)
        .map_or_else(Vec::new, Iterator::collect);

    file_auth::add_delete_users(password_file, &add_users, &delete_users)
}

/// Entry point of server
pub fn handle_cmdline() -> Result<(), Error> {
    let matches = get_cmdline().get_matches();
    if let Some((SUBCMD_PASSWORD, sub_matches)) = matches.subcommand() {
        return handle_password_subcmd(sub_matches);
    }

    let config_file = if let Some(config_file) = matches.value_of(OPT_CONFIG) {
        Some(config_file)
    } else if Path::new(DEFAULT_CONFIG).exists() {
        Some(DEFAULT_CONFIG)
    } else {
        None
    };

    let config = if let Some(config_file) = config_file {
        let config_content = std::fs::read_to_string(config_file).map_err(|err| {
            Error::from_string(
                ErrorKind::ConfigError,
                format!("Failed to read config file {}, err: {:?}", config_file, err),
            )
        })?;
        let config: Config = toml::from_str(&config_content).map_err(|err| {
            Error::from_string(
                ErrorKind::ConfigError,
                format!("Invalid toml config file {}, err: {:?}", config_file, err),
            )
        })?;

        if matches.is_present(OPT_TEST) {
            if let Err(err) = config.validate(false) {
                eprintln!("Failed to validate config file!");
                return Err(err);
            }
            println!("The configuration file {} syntax is Ok", config_file);
            return Ok(());
        }
        config
    } else {
        Config::default()
    };

    init_log(config.log())?;

    let mut server = ServerContext::new(config);

    if matches.is_present(OPT_STOP) {
        return server.send_stop_signal();
    }

    if matches.is_present(OPT_RELOAD) {
        return server.send_reload_signal();
    }

    let runtime = Runtime::new()?;
    server.run_loop(&runtime)
}

/// Run server with predefined config.
///
/// Useful for integration tests.
///
/// # Errors
///
/// Returns error if:
/// - Failed to init log module
/// - Failed to crate a runtime instance
/// - Failed to start server
#[allow(clippy::module_name_repetitions)]
pub fn run_server_with_config(config: Config) -> Result<(), Error> {
    init_log(config.log())?;
    let mut server = ServerContext::new(config);
    let runtime = Runtime::new()?;
    server.run_loop(&runtime)
}
