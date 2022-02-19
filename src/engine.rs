use std::fs::File;
use std::io::{stdin, BufRead, BufReader, Lines, StdinLock};
use std::iter::Flatten;
use std::path::PathBuf;

use log::{debug, info};

use crate::{HardModeWordPicker, DEFAULT_WORD_LIST};

pub struct Exit;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Reply {
    Success,
    Miss,
    Partial,
}

pub struct Engine {
    word_list: Vec<String>,
    // lines: Flatten<Lines<StdinLock<'a>>>,
}

impl Engine {
    pub fn new(dictionary_path: Option<PathBuf>) -> Self {
        debug!("initializing engine");
        let word_list: Vec<_> = if let Some(dictionary_path) = dictionary_path {
            let file = File::open(dictionary_path).expect("unable to read dictionary");
            let reader = BufReader::new(file);

            reader.lines().flatten().collect()
        } else {
            DEFAULT_WORD_LIST.into_iter().map(|s| s.into()).collect()
        };
        info!("processed word list containing {} words", word_list.len());

        let stdin = stdin();
        let mut lines = stdin.lock().lines().flatten();

        Self { word_list }
    }
    pub fn start(&mut self) -> Result<!, Exit> {
        loop {
            if true {
                break Err(Exit);
            }
        }
    }
}
