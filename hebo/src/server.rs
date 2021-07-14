// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use clap::Arg;
use std::fs::File;
use std::io::{Read, Write};
use tokio::runtime::Runtime;
use tokio::sync::mpsc;

use crate::cache::Cache;
use crate::config::Config;
use crate::constants;
use crate::dispatcher::Dispatcher;
use crate::error::{Error, ErrorKind};
use crate::listener::Listener;
use crate::system::System;

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

    let config_file = matches
        .value_of("config")
        .unwrap_or(constants::DEFAULT_CONFIG);
    let config_content = std::fs::read_to_string(config_file)?;
    let config: Config = toml::from_str(&config_content).unwrap();

    if matches.is_present("test") {
        println!("The configuration file {} syntax is Ok", config_file);
        return Ok(());
    }

    let mut server = ServerContext::new(config);

    if matches.is_present("reload") {
        log::info!("Reload is present");
        return server.reload();
    }

    let runtime = Runtime::new()?;
    server.run_loop(runtime)
}

/// ServerContext manages lifetime of Dispatcher and Listeners.
/// All kernel signals are handled here.
#[derive(Debug)]
pub struct ServerContext {
    config: Config,
}

impl ServerContext {
    pub fn new(config: Config) -> ServerContext {
        ServerContext { config }
    }

    /// Notify server process to reload config by sending `SIGUSR1` signal.
    pub fn reload(&mut self) -> Result<(), Error> {
        log::info!("reload()");
        let mut fd = File::open(&self.config.general.pid_file)?;
        let mut pid_str = String::new();
        fd.read_to_string(&mut pid_str)?;
        log::info!("pid str: {}", pid_str);
        let pid = pid_str.parse::<i32>().map_err(|err| {
            Error::from_string(
                ErrorKind::PidError,
                format!(
                    "Failed to parse pid {} from file {:?}, err: {:?}",
                    pid_str, &self.config.general.pid_file, err
                ),
            )
        })?;
        nc::kill(pid, nc::SIGUSR1).map_err(|err| {
            Error::from_string(
                ErrorKind::PidError,
                format!(
                    "Failed to notify process {}, got {}",
                    pid,
                    nc::strerror(err)
                ),
            )
        })?;
        Ok(())
    }

    fn write_pid(&self) -> Result<(), Error> {
        let pid = std::process::id();
        let mut fd = File::create(&self.config.general.pid_file)?;
        write!(fd, "{}", pid)?;
        Ok(())
    }

    /// Init modules and run tokio runtime.
    pub fn run_loop(&mut self, runtime: Runtime) -> Result<(), Error> {
        self.write_pid()?;

        runtime.block_on(async {
            let (listeners_to_dispatcher_sender, listeners_to_dispatcher_receiver) =
                mpsc::channel(constants::CHANNEL_CAPACITY);
            let mut dispatcher_to_listener_senders = Vec::new();
            let mut handles = Vec::new();
            let mut listener_id: u32 = 0;

            for l in self.config.listeners.clone() {
                let (dispatcher_to_listener_sender, dispatcher_to_listener_receiver) =
                    mpsc::channel(constants::CHANNEL_CAPACITY);
                dispatcher_to_listener_senders.push((listener_id, dispatcher_to_listener_sender));
                let mut listener = Listener::bind(
                    listener_id,
                    &l,
                    listeners_to_dispatcher_sender.clone(),
                    dispatcher_to_listener_receiver,
                )
                .await
                .expect(&format!("Failed to listen at {:?}", l));
                let handle = runtime.spawn(async move {
                    listener.run_loop().await;
                });
                handles.push(handle);
                listener_id += 1;
            }

            let (system_to_dispatcher_sender, system_to_dispatcher_receiver) =
                mpsc::channel(constants::CHANNEL_CAPACITY);
            let mut system = System::new(
                self.config.general.sys_interval,
                system_to_dispatcher_sender,
            );
            let system_handle = runtime.spawn(async move {
                system.run_loop().await;
            });
            handles.push(system_handle);

            let (cache_to_dispatcher_sender, cache_to_dispatcher_receiver) =
                mpsc::channel(constants::CHANNEL_CAPACITY);
            let (dispatcher_to_cache_sender, dispatcher_to_cache_receiver) =
                mpsc::channel(constants::CHANNEL_CAPACITY);
            let mut cache = Cache::new(cache_to_dispatcher_sender, dispatcher_to_cache_receiver);
            let cache_handle = runtime.spawn(async move {
                cache.run_loop().await;
            });
            handles.push(cache_handle);

            let mut dispatcher = Dispatcher::new(
                // listeners module
                dispatcher_to_listener_senders,
                listeners_to_dispatcher_receiver,
                // system module
                system_to_dispatcher_receiver,
                // cache module
                dispatcher_to_cache_sender,
                cache_to_dispatcher_receiver,
            );
            let dispatcher_handle = runtime.spawn(async move {
                dispatcher.run_loop().await;
            });
            handles.push(dispatcher_handle);

            for handle in handles {
                let _ret = handle.await;
            }
        });

        Ok(())
    }
}
