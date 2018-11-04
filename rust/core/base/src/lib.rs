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
use std::sync::Mutex;

lazy_static! {
  static ref DID_INIT: Mutex<bool> = Mutex::new(false);
}

#[derive(Clone, Debug)]
pub struct LogLevelParsable(pub log::LevelFilter);

impl zcfg::ConfigParseable for LogLevelParsable {
  type Output = LogLevelParsable;
  fn parse_from_str(s: &str) -> Result<Self::Output, zcfg::ParseErr> {
    match s {
      "trace" | "Trace" => Ok(LogLevelParsable(log::LevelFilter::Trace)),
      "debug" | "Debug" => Ok(LogLevelParsable(log::LevelFilter::Debug)),
      "info" | "Info" => Ok(LogLevelParsable(log::LevelFilter::Info)),
      "warn" | "Warn" => Ok(LogLevelParsable(log::LevelFilter::Warn)),
      "error" | "Error" => Ok(LogLevelParsable(log::LevelFilter::Error)),
      _ => Err("Unknown LogLevel value".to_owned()),
    }
  }
}

define_pub_cfg!(
  log_level,
  ::LogLevelParsable,
  ::LogLevelParsable(::log::LevelFilter::Info),
  "What log level to emit logs to"
);


pub fn init() {
  // Check for double init and early exit
  {
    let mut did_init = DID_INIT.lock().unwrap();
    if *did_init {
      return
    }

    *did_init = true;
  }

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
