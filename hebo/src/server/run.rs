// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use clap::Arg;
use std::path::Path;
use tokio::runtime::Runtime;

use super::ServerContext;
use crate::config::Config;
use crate::error::{Error, ErrorKind};
use crate::log::init_log;

pub const DEFAULT_CONFIG: &str = "/etc/hebo/hebo.toml";
const OPT_CONFIG: &str = "config";
const OPT_RELOAD: &str = "reload";
const OPT_STOP: &str = "stop";
const OPT_TEST: &str = "test";

/// Entry point of server
pub fn run_server() -> Result<(), Error> {
    let matches = clap::Command::new("Hebo")
        .version("0.1.0")
        .author("Xu Shaohua <shaohua@biofan.org>")
        .about("High Performance MQTT Server")
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
        .get_matches();

    let config_file = if let Some(config_file) = matches.value_of(OPT_CONFIG) {
        Some(config_file)
    } else if Path::new(DEFAULT_CONFIG).exists() {
        Some(DEFAULT_CONFIG)
    } else {
        None
    };

    let config = if let Some(config_file) = config_file {
        let config_content = std::fs::read_to_string(config_file)?;
        let config: Config = toml::from_str(&config_content).map_err(|err| {
            Error::from_string(ErrorKind::ConfigError, format!("Invalid config: {:?}", err))
        })?;

        config.validate()?;

        if matches.is_present(OPT_TEST) {
            println!("The configuration file {} syntax is Ok", config_file);
            return Ok(());
        }
        config
    } else {
        Config::default()
    };

    init_log(&config.log())?;

    let mut server = ServerContext::new(config);

    if matches.is_present(OPT_RELOAD) {
        return server.send_reload_signal();
    }

    if matches.is_present(OPT_STOP) {
        return server.send_stop_signal();
    }

    let runtime = Runtime::new()?;
    server.run_loop(runtime)
}

/// Run server with predefined config.
///
/// Useful for integration tests.
pub fn run_server_with_config(config: Config) -> Result<(), Error> {
    init_log(&config.log())?;
    let mut server = ServerContext::new(config);
    let runtime = Runtime::new()?;
    server.run_loop(runtime)
}
