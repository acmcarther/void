#![feature(used)]
extern crate chrono;
extern crate fern;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate zcfg;
extern crate zcfg_flag_parser;

use std::env;

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

define_pub_cfg!(
  log_level,
  ::LogLevelParsable,
  ::LogLevelParsable(::log::LogLevelFilter::Info),
  "What log level to emit logs to"
);

pub fn init() {
  zcfg_flag_parser::FlagParser::new()
    .parse_from_args(env::args().skip(1))
    .unwrap();
  let LogLevelParsable(log_level) = log_level::CONFIG.get_value();
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

  info!("Init complete!");
}
