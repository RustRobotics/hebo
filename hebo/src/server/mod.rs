// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

//! `ServerContex` is the main entry pointer of hebo server.

use std::fs::File;
use std::io::{Read, Write};
use tokio::runtime::Runtime;
use tokio::signal::unix::{signal, SignalKind};
use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::commands::{
    DashboardToServerContexCmd, ServerContextToAclCmd, ServerContextToAuthCmd,
    ServerContextToBackendsCmd, ServerContextToBridgeCmd, ServerContextToGatewayCmd,
    ServerContextToMetricsCmd, ServerContextToRuleEngineCmd,
};
use crate::config::Config;
use crate::error::{Error, ErrorKind};

mod dashboard;
mod init;
pub mod run;

pub const CHANNEL_CAPACITY: usize = 16;

/// `ServerContext` manages lifetime of Dispatcher and Listeners.
///
/// All kernel signals are handled here.
// TODO(Shaohua): Remove this attribute mark.
#[allow(dead_code)]
#[allow(clippy::module_name_repetitions)]
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
    #[must_use]
    pub fn new(config: Config) -> Self {
        let (dashboard_sender, dashboard_receiver) = mpsc::channel(CHANNEL_CAPACITY);
        let (acl_sender, acl_receiver) = mpsc::channel(CHANNEL_CAPACITY);
        let (auth_sender, auth_receiver) = mpsc::channel(CHANNEL_CAPACITY);
        let (backends_sender, backends_receiver) = mpsc::channel(CHANNEL_CAPACITY);
        let (bridge_sender, bridge_receiver) = mpsc::channel(CHANNEL_CAPACITY);
        let (gateway_sender, gateway_receiver) = mpsc::channel(CHANNEL_CAPACITY);
        let (metrics_sender, metrics_receiver) = mpsc::channel(CHANNEL_CAPACITY);
        let (rule_engine_sender, rule_engine_receiver) = mpsc::channel(CHANNEL_CAPACITY);

        Self {
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

    /// Send `SIGUSR1` signal to running process.
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Failed to read pid from file
    /// - Failed to find that process
    pub fn send_reload_signal(&mut self) -> Result<(), Error> {
        #[cfg(target_os = "linux")]
        return self.send_signal(nc::SIGUSR1);

        #[cfg(not(target_os = "linux"))]
        return self.send_signal(0);
    }

    /// Send `SIGTERM` signal to running process.
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Failed to read pid from file
    /// - Failed to find that process
    pub fn send_stop_signal(&mut self) -> Result<(), Error> {
        #[cfg(target_os = "linux")]
        return self.send_signal(nc::SIGTERM);

        #[cfg(not(target_os = "linux"))]
        return self.send_signal(0);
    }

    /// Notify server process to reload config by sending a signal.
    #[cfg(not(target_os = "linux"))]
    fn send_signal(&mut self, _sig: i32) -> Result<(), Error> {
        Ok(())
    }

    /// Notify server process to reload config by sending a signal.
    #[cfg(target_os = "linux")]
    fn send_signal(&mut self, sig: i32) -> Result<(), Error> {
        log::info!("send_signal() {}", sig);
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

        unsafe {
            nc::kill(pid, sig).map_err(|err| {
                Error::from_string(
                    ErrorKind::PidError,
                    format!(
                        "Failed to notify process {}, got {}",
                        pid,
                        nc::strerror(err)
                    ),
                )
            })?;
        }
        Ok(())
    }

    fn write_pid(&self) -> Result<(), Error> {
        let pid = std::process::id();
        let mut fd = File::create(&self.config.general().pid_file()).map_err(|err| {
            Error::from_string(
                ErrorKind::IoError,
                format!(
                    "Failed to write pid to file {:?}, got err: {:?}",
                    &self.config.general().pid_file(),
                    err
                ),
            )
        })?;
        write!(fd, "{}", pid)?;
        Ok(())
    }

    #[cfg(not(target_os = "linux"))]
    fn set_uid(&self) -> Result<(), Error> {
        Ok(())
    }

    #[cfg(target_os = "linux")]
    fn set_uid(&self) -> Result<(), Error> {
        let euid = unsafe { nc::geteuid() };
        if euid == 0 {
            // For root only.
            let user = self.config.general().user();
            users::get_user_by_name(user).map_or_else(
                || {
                    Err(Error::from_string(
                        ErrorKind::ConfigError,
                        format!("Failed to get user entry by name: {}", user),
                    ))
                },
                |user| {
                    let real_uid = user.uid();
                    if let Err(errno) = unsafe { nc::setuid(real_uid) } {
                        Err(Error::from_string(
                            ErrorKind::ConfigError,
                            format!(
                                "Failed to setuid({}), got err: {}",
                                real_uid,
                                nc::strerror(errno)
                            ),
                        ))
                    } else {
                        Ok(())
                    }
                },
            )
        } else {
            // Normal user, do nothing.
            Ok(())
        }
    }

    /// Init modules and run tokio runtime.
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Server config is invalid
    /// - Failed to write pid to file
    /// - Failed to init inner modules
    pub fn run_loop(&mut self, runtime: &Runtime) -> Result<(), Error> {
        if let Err(err) = self.config.validate(true) {
            eprintln!("Failed to validate config file!");
            return Err(err);
        }

        self.write_pid()?;

        runtime.block_on(async {
            self.init_modules(runtime).await?;
            self.run_inner_loop().await
        })
    }

    async fn run_inner_loop(&mut self) -> Result<(), Error> {
        log::info!("ServerContext::run_inner_loop()");
        let mut sigusr1_stream = signal(SignalKind::user_defined1())?;
        let mut sigterm_stream = signal(SignalKind::terminate())?;
        let mut sigquit_stream = signal(SignalKind::quit())?;
        let mut sigint_stream = signal(SignalKind::interrupt())?;

        loop {
            tokio::select! {
                Some(cmd) = self.dashboard_receiver.recv() => {
                    if let Err(err) = self.handle_dashboard_cmd(cmd).await {
                        log::error!("Failed to handle dashboard cmd: {:?}", err);
                    }
                }
                Some(_) = sigusr1_stream.recv() => {
                    log::info!("Realod config");
                    // TODO(Shaohua): Reload config and send new config to other apps.
                },
                Some(_) = sigterm_stream.recv() => {
                    log::info!("Quit with SIGTERM");
                    break;
                }
                Some(_) = sigquit_stream.recv() => {
                    log::info!("Quit with SIGQUIT");
                    break;
                }
                Some(_) = sigint_stream.recv() => {
                    log::info!("Quit with SIGINT");
                    break;
                }
            }
        }

        Ok(())
    }
}
