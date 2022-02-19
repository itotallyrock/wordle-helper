use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use arrayvec::ArrayVec;
use log::{debug, info, trace};

use crate::parser::{Parser, ReadTurnFlags};
use crate::word_picker::HardModeWordPicker;
use crate::{DEFAULT_WORD_LIST, MAX_GUESSES};

/// Maximum number of potential solution words to present after a turn
const BEST_WORDS_LEN: usize = 10;

/// Flag for special exit input
pub struct Exit;

/// Manages the word picker and handles input
pub struct Engine {
    word_list: Vec<String>,
    parser: Parser,
}

impl Engine {
    /// Create a new engine, can be given a path to a dictionary text file
    pub fn new(dictionary_path: Option<PathBuf>) -> Self {
        debug!(
            "initializing engine with {}",
            if let Some(path) = &dictionary_path {
                path.to_str().unwrap_or("<non-unicode dictionary path>")
            } else {
                "default dictionary"
            }
        );
        let word_list: Vec<_> = if let Some(dictionary_path) = dictionary_path {
            let file = File::open(dictionary_path).expect("unable to read dictionary");
            let reader = BufReader::new(file);

            let word_list: Vec<_> = reader.lines().flatten().collect();
            info!("processed word list containing {} words", word_list.len());

            word_list
        } else {
            DEFAULT_WORD_LIST.into_iter().map(|s| s.into()).collect()
        };

        Self {
            word_list,
            parser: Parser::new(),
        }
    }

    /// Runs the engine, continuously reads input and present solutions until exiting
    pub fn start(&mut self) -> Result<!, Exit> {
        debug!("starting engine");
        loop {
            let mut word_picker = HardModeWordPicker::new(self.word_list.clone());
            trace!("created fresh word picker from dictionary");

            println!(
                "\nStarting new game - {} Potential Solutions",
                self.word_list.len()
            );

            for turn_index in 0..MAX_GUESSES {
                trace!("starting new turn {}", turn_index);
                let turn = match self.parser.read_turn() {
                    Ok(turn) => Ok(turn),
                    Err(err) => match err {
                        ReadTurnFlags::Exit(e) => Err(e),
                        ReadTurnFlags::Win => break,
                    },
                }?;
                // Remove words from word picker based on turn
                word_picker.take_turn(turn);
                self.print_best_guesses(&word_picker);

                // If no words are left the game is scratch (incorrect dictionary or invalid user input)
                if word_picker.remaining() == 0 {
                    break;
                }
            }
        }
    }
    fn print_best_guesses(&self, word_picker: &HardModeWordPicker) {
        const BEST_GUESS_SEPARATOR: &str = ", ";

        let remaining = word_picker.remaining();
        if remaining > 0 {
            // Print out best guesses
            let best_guesses = word_picker
                .top_10_words()
                .cloned()
                .collect::<ArrayVec<_, 10>>()
                .join(BEST_GUESS_SEPARATOR);

            println!(
                "{}/{} Best Guesses: {}",
                BEST_WORDS_LEN.min(remaining),
                remaining,
                best_guesses
            );
        } else {
            println!("0/{} Words remaining - Restarting", self.word_list.len());
        }
    }
}
