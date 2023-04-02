//#![windows_subsystem = "windows"]
#![allow(warnings)]
mod context;
mod game;
mod re;
mod screen;
mod storage;

use re::*;

pub static VERSION: &str = "1.1.0";

fn main() {
    Context::run();
}