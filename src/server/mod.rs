// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

//! ServerContex is the main entry pointer of hebo server.

use clap::Arg;
use std::fs::File;
use std::io::{Read, Write};
use tokio::runtime::Runtime;
use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::commands::{
    DashboardToServerContexCmd, ServerContextToAclCmd, ServerContextToAuthCmd,
    ServerContextToBackendsCmd, ServerContextToBridgeCmd, ServerContextToGatewayCmd,
    ServerContextToMetricsCmd, ServerContextToRuleEngineCmd,
};
use crate::config::Config;
use crate::error::{Error, ErrorKind};
use crate::log::init_log;

mod dashboard;
mod init;

pub const DEFAULT_CONFIG: &str = "/etc/hebo/hebo.toml";
// TODO(Shaohua): Move to consts module.
pub const CHANNEL_CAPACITY: usize = 16;

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

    // dashboard -> server_ctx
    dashboard_sender: Option<Sender<DashboardToServerContexCmd>>,
    dashboard_receiver: Receiver<DashboardToServerContexCmd>,

    // server_ctx -> acl
    acl_sender: Sender<ServerContextToAclCmd>,
    acl_receiver: Option<Receiver<ServerContextToAclCmd>>,

    // server_ctx -> auth
    auth_sender: Sender<ServerContextToAuthCmd>,
    auth_receiver: Option<Receiver<ServerContextToAuthCmd>>,

    // server_ctx -> backends
    backends_sender: Sender<ServerContextToBackendsCmd>,
    backends_receiver: Option<Receiver<ServerContextToBackendsCmd>>,

    // server_ctx -> bridge
    bridge_sender: Sender<ServerContextToBridgeCmd>,
    bridge_receiver: Option<Receiver<ServerContextToBridgeCmd>>,

    // server_ctx -> gateway
    gateway_sender: Sender<ServerContextToGatewayCmd>,
    gateway_receiver: Option<Receiver<ServerContextToGatewayCmd>>,

    // server_ctx -> metrics
    metrics_sender: Sender<ServerContextToMetricsCmd>,
    metrics_receiver: Option<Receiver<ServerContextToMetricsCmd>>,

    // server_ctx -> rule_engine
    rule_engine_sender: Sender<ServerContextToRuleEngineCmd>,
    rule_engine_receiver: Option<Receiver<ServerContextToRuleEngineCmd>>,
}

impl ServerContext {
    pub fn new(config: Config) -> ServerContext {
        let (dashboard_sender, dashboard_receiver) = mpsc::channel(CHANNEL_CAPACITY);
        let (acl_sender, acl_receiver) = mpsc::channel(CHANNEL_CAPACITY);
        let (auth_sender, auth_receiver) = mpsc::channel(CHANNEL_CAPACITY);
        let (backends_sender, backends_receiver) = mpsc::channel(CHANNEL_CAPACITY);
        let (bridge_sender, bridge_receiver) = mpsc::channel(CHANNEL_CAPACITY);
        let (gateway_sender, gateway_receiver) = mpsc::channel(CHANNEL_CAPACITY);
        let (metrics_sender, metrics_receiver) = mpsc::channel(CHANNEL_CAPACITY);
        let (rule_engine_sender, rule_engine_receiver) = mpsc::channel(CHANNEL_CAPACITY);

        ServerContext {
            config,

            dashboard_sender: Some(dashboard_sender),
            dashboard_receiver,

            acl_sender,
            acl_receiver: Some(acl_receiver),

            auth_sender,
            auth_receiver: Some(auth_receiver),

            backends_sender,
            backends_receiver: Some(backends_receiver),

            bridge_sender,
            bridge_receiver: Some(bridge_receiver),

            gateway_sender,
            gateway_receiver: Some(gateway_receiver),

            metrics_sender,
            metrics_receiver: Some(metrics_receiver),

            rule_engine_sender,
            rule_engine_receiver: Some(rule_engine_receiver),
        }
    }

    /// Notify server process to reload config by sending `SIGUSR1` signal.
    pub fn reload(&mut self) -> Result<(), Error> {
        log::info!("reload()");
        let mut fd = File::open(&self.config.general().pid_file())?;
        let mut pid_str = String::new();
        fd.read_to_string(&mut pid_str)?;
        log::info!("pid str: {}", pid_str);
        let pid = pid_str.parse::<i32>().map_err(|err| {
            Error::from_string(
                ErrorKind::PidError,
                format!(
                    "Failed to parse pid {} from file {:?}, err: {:?}",
                    pid_str,
                    &self.config.general().pid_file(),
                    err
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
        let mut fd = File::create(&self.config.general().pid_file())?;
        write!(fd, "{}", pid)?;
        Ok(())
    }

    /// Init modules and run tokio runtime.
    pub fn run_loop(&mut self, runtime: Runtime) -> Result<(), Error> {
        self.write_pid()?;

        runtime.block_on(async {
            self.init_modules(&runtime).await?;
            self.run_inner_loop().await
        })
    }

    async fn run_inner_loop(&mut self) -> Result<(), Error> {
        log::info!("ServerContext::run_inner_loop()");
        loop {
            tokio::select! {
                Some(cmd) = self.dashboard_receiver.recv() => {
                    if let Err(err) = self.handle_dashboard_cmd(cmd).await {
                        log::error!("Failed to handle dashboard cmd: {:?}", err);
                    }
                }
            }
        }

        // TODO(Shaohua): Break main loop
        #[allow(unreachable_code)]
        Ok(())
    }
}
