#![feature(used)]
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;
#[macro_use] extern crate zcfg;
extern crate chrono;
extern crate fern;
extern crate monitoring;
extern crate zcfg_flag_parser;

use std::env;
use std::thread;

#[derive(Clone, Debug)]
pub struct LogLevelParsable(pub log::LogLevelFilter);

impl zcfg::ConfigParseable for LogLevelParsable {
  type Output = LogLevelParsable;
  fn parse_from_str(s: &str) -> Result<Self::Output, zcfg::ParseErr> {
    match s {
      "trace" | "Trace" => Ok(LogLevelParsable(log::LogLevelFilter::Trace)),
      "debug" | "Debug" => Ok(LogLevelParsable(log::LogLevelFilter::Debug)),
      "info" | "Info" => Ok(LogLevelParsable(log::LogLevelFilter::Info)),
      "warn" | "Warn" => Ok(LogLevelParsable(log::LogLevelFilter::Warn)),
      "error" | "Error" => Ok(LogLevelParsable(log::LogLevelFilter::Error)),
      _ => Err("Unknown LogLevel value".to_owned()),
    }
  }
}

define_cfg!(init_log_level,
            ::LogLevelParsable,
            ::LogLevelParsable(::log::LogLevelFilter::Debug),
            "What log level to emit logs to");

define_cfg!(init_start_monitoring_daemon,
            bool,
            true,
            "Whether or not to start the monitor daemon");

pub fn init_void() {
  zcfg_flag_parser::FlagParser::new().parse_from_args(env::args().skip(1)).unwrap();
  let LogLevelParsable(log_level) =
    init_log_level::CONFIG.get_value();
  fern::Dispatch::new()
    // Perform allocation-free log formatting
    .format(|out, message, record| {
        out.finish(format_args!("{}[{}][{}] {}",
            chrono::Local::now()
                .format("[%Y-%m-%d][%H:%M:%S]"),
            record.target(),
            record.level(),
            message))
    })
    // Add blanket level filter -
    .level(log_level)
    // Output to stdout, files, and other Dispatch configs
    .chain(std::io::stdout())
    // Apply globally
    .apply()
    .unwrap();
  debug!("Logger initialization complete.");

  monitoring::init();

  if init_start_monitoring_daemon::CONFIG.get_value() {
    debug!("Spawning monitoring daemon.");
    thread::spawn(move || {
      monitoring::MonitoringService::default().run_forever()
    });
  } else {
    debug!("Not spawning monitoring daemon due to configuration.");
  }

  info!("Init complete!");
}
