// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use super::config::Config;
use super::server_context::ServerContext;
use clap::Arg;
use std::io;

const DEFAULT_CONFIG: &'static str = "/etc/hebo/hebo.toml";

pub async fn run_server() -> io::Result<()> {
    let matches = clap::App::new("Hebo")
        .version("0.1.0")
        .author("Xu Shaohua <shaohua@biofan.org>")
        .about("Distributed MQTT Broker")
        .arg(
            Arg::with_name("config_file")
                .short("c")
                .long("config")
                .takes_value(true)
                .help("Specify config file path"),
        )
        .arg(
            Arg::with_name("test")
                .short("t")
                .long("test")
                .takes_value(false)
                .help("Test config file"),
        )
        .get_matches();

    if matches.value_of("test").is_some() {
        let config_file = matches.value_of("config_file").unwrap_or(DEFAULT_CONFIG);
        log::info!("Reading config file: {}", config_file);
        let config_content = std::fs::read_to_string(config_file)?;
        let _config: Config = toml::from_str(&config_content).unwrap();
        return Ok(());
    }

    let config_file = matches.value_of("config_file").unwrap_or(DEFAULT_CONFIG);
    let config_content = std::fs::read_to_string(config_file)?;
    let config: Config = toml::from_str(&config_content).unwrap();

    let mut server = ServerContext::new(config);
    server.run_loop().await
}
