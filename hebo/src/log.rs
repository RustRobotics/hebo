// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use log::LevelFilter;
use log4rs::{
    append::console::ConsoleAppender,
    append::console::Target,
    append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller,
    append::rolling_file::policy::compound::trigger::size::SizeTrigger,
    append::rolling_file::policy::compound::CompoundPolicy,
    append::rolling_file::RollingFileAppender,
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};

use crate::config::{self, LogLevel};
use crate::error::{Error, ErrorKind};

const LOG_FILE_SIZE: u64 = 64 * 1024 * 1024;
const ROLLER_PATTERN: &str = ".{}.gz";
const ROLLER_COUNT: u32 = 16;
const STDOUT_NAME: &str = "stdout";
const ROLLER_NAME: &str = "roller";

const fn get_log_level(level: LogLevel) -> LevelFilter {
    match level {
        LogLevel::Off => LevelFilter::Off,
        LogLevel::Error => LevelFilter::Error,
        LogLevel::Warn => LevelFilter::Warn,
        LogLevel::Info => LevelFilter::Info,
        LogLevel::Debug => LevelFilter::Debug,
        LogLevel::Trace => LevelFilter::Trace,
    }
}

/// Initialize log module.
///
/// # Errors
///
/// Returns error if:
/// - Failed to init rolling pattern or rolling appender
/// - Failed to init log4rs
#[allow(clippy::module_name_repetitions)]
pub fn init_log(log_conf: &config::Log) -> Result<(), Error> {
    let mut config_builder = Config::builder();
    let mut root_builder = Root::builder();
    if log_conf.console_log() {
        let stdout = ConsoleAppender::builder()
            .encoder(Box::new(PatternEncoder::new(
                "{d(%Y-%m-%d %H:%M:%S)} {l} {M}:{L} - {m}{n}",
            )))
            .target(Target::Stderr)
            .build();
        config_builder =
            config_builder.appender(Appender::builder().build(STDOUT_NAME, Box::new(stdout)));
        root_builder = root_builder.appender(STDOUT_NAME);
    }

    if let Some(log_file) = log_conf.log_file() {
        let roller_pattern = log_file.to_string() + ROLLER_PATTERN;
        let roller = FixedWindowRoller::builder()
            .build(&roller_pattern, ROLLER_COUNT)
            .map_err(|err| {
                Error::from_string(
                    ErrorKind::LoggerError,
                    format!("Failed to init roller pattern, {err:?}"),
                )
            })?;
        let rolling_policy = Box::new(CompoundPolicy::new(
            Box::new(SizeTrigger::new(LOG_FILE_SIZE)),
            Box::new(roller),
        ));
        let requests = RollingFileAppender::builder()
            .build(log_file, rolling_policy)
            .map_err(|err| {
                Error::from_string(
                    ErrorKind::LoggerError,
                    format!("Failed to init roller appender, {err:?}"),
                )
            })?;

        config_builder =
            config_builder.appender(Appender::builder().build(ROLLER_NAME, Box::new(requests)));
        root_builder = root_builder.appender(ROLLER_NAME);
    }

    let log_level = get_log_level(log_conf.log_level());
    let config = config_builder
        .build(root_builder.build(log_level))
        .map_err(|err| {
            Error::from_string(
                ErrorKind::LoggerError,
                format!("Failed to build log4rs config, {err:?}"),
            )
        })?;

    log4rs::init_config(config).map_err(|err| {
        Error::from_string(
            ErrorKind::LoggerError,
            format!("Failed to init log4rs, {err:?}"),
        )
    })?;
    Ok(())
}
