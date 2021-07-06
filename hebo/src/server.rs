// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use clap::Arg;
use tokio::runtime::Runtime;

use crate::config::Config;
use crate::error::Error;
use crate::listener;

const DEFAULT_CONFIG: &'static str = "/etc/hebo/hebo.toml";

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

    let runtime = Runtime::new()?;
    let mut server = ServerContext::new(config);
    server.run_loop(runtime)
}

/// ServerContext manages lifetime of Storage and Listeners.
/// All kernel signals are handled here.
#[derive(Debug)]
pub struct ServerContext {
    config: Config,
}

impl ServerContext {
    pub fn new(config: Config) -> ServerContext {
        ServerContext { config }
    }

    pub fn run_loop(&mut self, runtime: Runtime) -> Result<(), Error> {
        let mut handles = Vec::new();

        for l in self.config.listeners.clone() {
            let handle = runtime.spawn(async move {
                let mut listener = listener::Listener::bind(&l)
                    .await
                    .expect(&format!("Failed to listen at {:?}", l));
                listener.run_loop().await;
            });
            handles.push(handle);
        }

        //runtime.spawn(async {
        //    if let Some(cmd) = self.connection_rx.recv() {
        //        self.route_cmd(cmd).await;
        //    }
        //});
        for handle in handles {
            //handle.await;
        }
        Ok(())
    }
}
