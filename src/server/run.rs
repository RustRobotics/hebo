// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use clap::Arg;
use tokio::runtime::Runtime;

use super::ServerContext;
use crate::config::Config;
use crate::error::{Error, ErrorKind};
use crate::log::init_log;

pub const DEFAULT_CONFIG: &str = "/etc/hebo/hebo.toml";

/// Entry point of server
pub fn run_server() -> Result<(), Error> {
    let matches = clap::App::new("Hebo")
        .version("0.1.0")
        .author("Xu Shaohua <shaohua@biofan.org>")
        .about("High Performance MQTT Server")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("config_file")
                .takes_value(true)
                .help("Specify config file path"),
        )
        .arg(
            Arg::with_name("reload")
                .short("r")
                .long("reload")
                .takes_value(false)
                .help("Reload config"),
        )
        .arg(
            Arg::with_name("test")
                .short("t")
                .long("test")
                .takes_value(false)
                .help("Test config file"),
        )
        .get_matches();

    let config_file = matches.value_of("config").unwrap_or(DEFAULT_CONFIG);
    let config_content = std::fs::read_to_string(config_file)?;
    let config: Config = toml::from_str(&config_content).map_err(|err| {
        Error::from_string(ErrorKind::ConfigError, format!("Invalid config: {:?}", err))
    })?;

    if matches.is_present("test") {
        println!("The configuration file {} syntax is Ok", config_file);
        return Ok(());
    }

    init_log(&config.log())?;

    let mut server = ServerContext::new(config);

    if matches.is_present("reload") {
        log::info!("Reload is present");
        return server.send_reload();
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
