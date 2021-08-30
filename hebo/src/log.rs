// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::config::{Appender, Config, Logger, Root};

use crate::config;
use crate::error::Error;

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
    let stdout = ConsoleAppender::builder().build();

    let rolling_policy = CompoundPolicy::new();
    let requests = RollingFileAppender::builder()
        .build(&log_conf.log_file, rolling_policy)
        .unwrap();

    let log_level = get_log_level(log_conf.level);
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("requests", Box::new(requests)))
        .logger(
            Logger::builder()
                .appender("requests")
                .additive(false)
                .build("app::requests", log_level),
        )
        .build(Root::builder().appender("stdout").build(log_level))
        .unwrap();

    let handle = log4rs::init_config(config).unwrap();
    Ok(())
}
