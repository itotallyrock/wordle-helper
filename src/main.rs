#![feature(never_type)]
#![feature(stdio_locked)]

use std::path::PathBuf;

use clap::{ArgEnum, Parser};
use log::{debug, LevelFilter};
use simple_logger::SimpleLogger;

use crate::default_word_list::DEFAULT_WORD_LIST;
use crate::engine::Engine;
use crate::game::{MAX_GUESSES, NUM_LETTERS};

mod default_word_list;
mod engine;
mod game;
mod parser;
mod word_picker;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, parse(from_os_str), value_name = "FILE")]
    dictionary: Option<PathBuf>,
    #[clap(short, long, arg_enum, value_name = "LEVEL")]
    log_level: Option<MyLogLevel>,
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Copy, Clone, ArgEnum)]
enum MyLogLevel {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

struct FailedToStartLogger;

fn setup_logger(log_level: Option<MyLogLevel>) -> Result<(), FailedToStartLogger> {
    let level_filter = log_level
        .map(|custom_level| match custom_level {
            MyLogLevel::Off => LevelFilter::Off,
            MyLogLevel::Error => LevelFilter::Error,
            MyLogLevel::Warn => LevelFilter::Warn,
            MyLogLevel::Info => LevelFilter::Info,
            MyLogLevel::Debug => LevelFilter::Debug,
            MyLogLevel::Trace => LevelFilter::Trace,
        })
        .unwrap_or(if cfg!(debug_assertions) {
            LevelFilter::Info
        } else {
            LevelFilter::Warn
        });

    SimpleLogger::new()
        .with_level(level_filter)
        .init()
        .map_err(|_| FailedToStartLogger)
}

fn main() {
    let Args {
        log_level,
        dictionary,
    } = Args::parse();

    if setup_logger(log_level).is_err() {
        eprintln!("failed to start logger");
        return;
    }

    Engine::new(dictionary).start();
    debug!("successfully exited");
}
