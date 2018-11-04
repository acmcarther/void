#![feature(used)]
extern crate base;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
extern crate netcode_io_sys as nio;
extern crate reliable_io_sys as rio;
#[macro_use]
extern crate zcfg;

pub mod global;
pub mod flags;
pub mod consts;

pub type ClientId = i64;
pub type ClientSlot = usize;

