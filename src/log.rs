// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use log::LevelFilter;
use log4rs::append::console;
use log4rs::append::rolling_file::policy::compound::{
    roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger, CompoundPolicy,
};
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

use crate::config;
use crate::error::{Error, ErrorKind};

const LOG_FILE_SIZE: u64 = 16 * 1024 * 1024;
const ROLLER_PATTERN: &str = ".{}.gz";
const ROLLER_COUNT: u32 = 10;

fn get_log_level(level: config::LogLevel) -> LevelFilter {
    match level {
        config::LogLevel::Off => LevelFilter::Off,
        config::LogLevel::Error => LevelFilter::Error,
        config::LogLevel::Warn => LevelFilter::Warn,
        config::LogLevel::Info => LevelFilter::Info,
        config::LogLevel::Debug => LevelFilter::Debug,
        config::LogLevel::Trace => LevelFilter::Trace,
    }
}

pub fn init_log(log_conf: &config::Log) -> Result<(), Error> {
    let stdout = console::ConsoleAppender::builder()
        .target(console::Target::Stderr)
        .encoder(Box::new(PatternEncoder::new("{d} {h({l})} - {m}{n}")))
        .build();

    let roller_pattern = log_conf.log_file().to_str().ok_or_else(|| {
        Error::from_string(
            ErrorKind::ConfigError,
            format!("config: Invalid log file {:?}", log_conf.log_file()),
        )
    })?;
    let roller_pattern = roller_pattern.to_string() + ROLLER_PATTERN;
    let roller = FixedWindowRoller::builder()
        .build(&roller_pattern, ROLLER_COUNT)
        .map_err(|err| {
            Error::from_string(
                ErrorKind::LoggerError,
                format!("Failed to init roller pattern, {:?}", err),
            )
        })?;
    let rolling_policy = Box::new(CompoundPolicy::new(
        Box::new(SizeTrigger::new(LOG_FILE_SIZE)),
        Box::new(roller),
    ));
    let requests = RollingFileAppender::builder()
        .build(log_conf.log_file(), rolling_policy)
        .map_err(|err| {
            Error::from_string(
                ErrorKind::LoggerError,
                format!("Failed to init roller appender, {:?}", err),
            )
        })?;

    let log_level = get_log_level(log_conf.log_level());

    const STDOUT_NAME: &str = "stdout";
    const ROLLER_NAME: &str = "roller";
    let config = Config::builder()
        .appender(Appender::builder().build(STDOUT_NAME, Box::new(stdout)))
        .appender(Appender::builder().build(ROLLER_NAME, Box::new(requests)))
        .build(
            Root::builder()
                .appenders([ROLLER_NAME, STDOUT_NAME])
                .build(log_level),
        )
        .map_err(|err| {
            Error::from_string(
                ErrorKind::LoggerError,
                format!("Failed to build log4rs config, {:?}", err),
            )
        })?;

    log4rs::init_config(config).map_err(|err| {
        Error::from_string(
            ErrorKind::LoggerError,
            format!("Failed to init log4rs, {:?}", err),
        )
    })?;
    Ok(())
}
