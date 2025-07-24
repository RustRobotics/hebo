// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use clap::Parser;
use std::path::Path;
use tokio::runtime::Runtime;

use super::ServerContext;
use crate::config::Config;
use crate::error::{Error, ErrorKind};
use crate::log::init_log;

pub const DEFAULT_CONFIG: &str = "/etc/hebo/hebo.toml";

#[derive(Debug, Parser)]
#[command(name = "Hebo")]
#[command(author = "Xu Shaohua <shaohua@biofan.org>")]
#[command(version = "0.3.2")]
#[command(about = "High Performance MQTT Server", long_about = None)]
struct Arguments {
    /// Specify config file path
    #[arg(short, long, value_name = "config_file")]
    config: Option<String>,

    /// Reload config.
    #[arg(short, long)]
    reload: bool,

    /// Stop server
    #[arg(short, long)]
    stop: bool,

    /// Test config file and exit.
    #[arg(short, long)]
    test: bool,
}

/*
fn handle_password_subcmd(matches: &ArgMatches) -> Result<(), Error> {
    let password_file = if let Some(file) = matches.value_of(OPT_PASSWORD_FILE) {
        file
    } else {
        return Err(Error::new(
            ErrorKind::ParameterError,
            "password_file is required",
        ));
    };

    if matches.contains_id(OPT_UPDATE) {
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
*/

/// Entry point of server
///
/// # Errors
///
/// Returns error if:
/// - Failed to read/parse config file
/// - Config file contains invalid options
/// - Failed to init log mod
pub fn handle_cmdline() -> Result<(), Error> {
    let args = Arguments::parse();

    let config_file = args.config.as_deref().map_or_else(
        || {
            if Path::new(DEFAULT_CONFIG).exists() {
                Some(DEFAULT_CONFIG)
            } else {
                None
            }
        },
        Some,
    );

    let config = if let Some(config_file) = config_file {
        let config_content = std::fs::read_to_string(config_file).map_err(|err| {
            Error::from_string(
                ErrorKind::ConfigError,
                format!("Failed to read config file {config_file}, err: {err:?}"),
            )
        })?;
        let config: Config = toml::from_str(&config_content).map_err(|err| {
            Error::from_string(
                ErrorKind::ConfigError,
                format!("Invalid toml config file {config_file}, err: {err:?}"),
            )
        })?;

        if args.test {
            if let Err(err) = config.validate(false) {
                eprintln!("Failed to validate config file!");
                return Err(err);
            }
            println!("The configuration file {config_file} syntax is Ok");
            return Ok(());
        }
        config
    } else {
        Config::default()
    };

    init_log(config.log())?;

    let mut server = ServerContext::new(config);

    if args.stop {
        return server.send_stop_signal();
    }

    if args.reload {
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
