#![feature(never_type)]

use std::borrow::Borrow;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{stdin, stdout, BufRead, BufReader, Lines, Read, Stdin, StdinLock, Write};
use std::iter::Flatten;
use std::mem::transmute;
use std::path::PathBuf;
use std::sync::Mutex;

use clap::{ArgEnum, Parser};
use log::{debug, info, warn, LevelFilter};
use simple_logger::SimpleLogger;

use parser::{REPLY_MISS, REPLY_PARTIAL, REPLY_SUCCESS};

use crate::default_word_list::DEFAULT_WORD_LIST;
use crate::engine::Engine;
use crate::game::{MAX_GUESSES, NUM_LETTERS};
use crate::word_picker::HardModeWordPicker;

mod cell;
mod default_word_list;
mod engine;
mod game;
mod parser;
mod turn;
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
    let args: Args = Args::parse();

    setup_logger(args.log_level);

    Engine::new(args.dictionary).start();
}
