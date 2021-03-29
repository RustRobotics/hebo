// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use super::config::Config;
use super::server_context::ServerContext;
use clap::Arg;

use crate::error::Error;

const DEFAULT_CONFIG: &str = "/etc/hebo/hebo.toml";

pub async fn run_server() -> Result<(), Error> {
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
            Arg::with_name("test")
                .short("t")
                .long("test")
                .takes_value(false)
                .help("Test config file"),
        )
        .get_matches();

    if matches.is_present("test") {
        let config_file = matches.value_of("config").unwrap_or(DEFAULT_CONFIG);
        let config_content = std::fs::read_to_string(config_file)?;
        let _config: Config = toml::from_str(&config_content).unwrap();
        println!("The configuration file {} syntax is Ok", config_file);
        return Ok(());
    }

    let config_file = matches.value_of("config").unwrap_or(DEFAULT_CONFIG);
    let config_content = std::fs::read_to_string(config_file)?;
    let config: Config = toml::from_str(&config_content).unwrap();

    let mut server = ServerContext::new(config);
    server.run_loop().await
}
