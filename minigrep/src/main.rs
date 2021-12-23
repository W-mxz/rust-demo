use std::{env, process};
use crate::config::{Config,};
mod config;
fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("解析参数时，发生了: {}错误", err);
        process::exit(1);
    });
    println!("ok");
    if let Err(e) = config::run(config) {
        eprintln!("Application error :{}", e);
        process::exit(1);
    }
}
