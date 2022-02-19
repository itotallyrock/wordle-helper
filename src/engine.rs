use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use log::{debug, info, trace};

use crate::parser::Parser;
use crate::word_picker::HardModeWordPicker;
use crate::{DEFAULT_WORD_LIST, MAX_GUESSES};

const BEST_WORDS_LEN: usize = 10;

pub struct Exit;

pub struct Engine {
    word_list: Vec<String>,
    parser: Parser,
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

        Self {
            word_list,
            parser: Parser::new(),
        }
    }
    pub fn start(&mut self) -> Result<!, Exit> {
        debug!("starting engine");
        loop {
            let mut word_picker = HardModeWordPicker::new(self.word_list.clone());
            trace!("created fresh word picker from dictionary");

            for turn_index in 0..MAX_GUESSES {
                trace!("starting new turn {}", turn_index);
                let turn = self.parser.read_turn()?;
                word_picker.take_turn(turn);

                // TODO: try to optimize this
                let best_guesses = word_picker
                    .top_10_words()
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(", ");
                println!(
                    "{}/{} Best Guesses: {}",
                    BEST_WORDS_LEN,
                    word_picker.remaining(),
                    best_guesses
                );
            }
        }
    }
}
