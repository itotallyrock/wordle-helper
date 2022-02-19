use std::io::{stdin_locked, stdout, BufRead, Lines, StdinLock, Write};
use std::iter::Flatten;

use crate::engine::Exit;
use crate::game::{GameCell, Guess, Reply};
use crate::game::{Response, Turn};
use crate::NUM_LETTERS;

const REPLY_SUCCESS: char = '+';
const REPLY_MISS: char = '.';
const REPLY_PARTIAL: char = '-';

pub enum ReadTurnFlags {
    Exit(Exit),
    Win,
}

pub struct Parser {
    lines: Flatten<Lines<StdinLock<'static>>>,
    winning_reply: String,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            lines: stdin_locked().lines().flatten(),
            winning_reply: String::from(REPLY_SUCCESS).repeat(NUM_LETTERS),
        }
    }
}

impl Parser {
    fn read_input(
        &mut self,
        input_name: &'static str,
        prompt: &str,
    ) -> Result<Guess, ReadTurnFlags> {
        loop {
            print!("{}: ", prompt);
            stdout()
                .lock()
                .flush()
                .expect("failed to flush prompt stdout (likely too long)");
            let input = if let Some(input) = self.lines.next() {
                input
            } else {
                eprintln!("failed to read {} from stdin: please try again", input_name);
                continue;
            };

            let guess = input.trim().to_ascii_lowercase();

            match guess.as_str() {
                "exit" | "quit" | "q" => break Err(ReadTurnFlags::Exit(Exit)),
                _ if guess == self.winning_reply => break Err(ReadTurnFlags::Win),
                _ => match Guess::from(&guess) {
                    // break Ok(Guess::from(&guess))
                    Ok(guess) => {
                        break Ok(guess);
                    }
                    Err(error_str) => {
                        eprintln!(
                            "illegal {}: expected {} characters but found {}",
                            input_name,
                            NUM_LETTERS,
                            error_str.element().len() + NUM_LETTERS
                        );
                    }
                },
            }
        }
    }

    fn read_guess(&mut self) -> Result<Guess, ReadTurnFlags> {
        const PROMPT: &str = "input guess";
        loop {
            let input = self.read_input("input", PROMPT)?;
            if input.to_ascii_lowercase() == self.winning_reply {}

            if input.chars().all(|c| c.is_ascii_alphabetic()) {
                break Ok(input);
            } else {
                eprintln!("illegal input: expected alphabetical characters");
            }
        }
    }

    fn read_reply(&mut self) -> Result<Response, ReadTurnFlags> {
        let prompt = format!(
            "input reply (miss: '{}', hit: '{}' partial: '{}')",
            REPLY_MISS, REPLY_SUCCESS, REPLY_PARTIAL
        );
        'outer: loop {
            let input = self.read_input("reply", &prompt)?;
            debug_assert_eq!(
                input.len(),
                NUM_LETTERS,
                "illegal reply was not checked in read_input"
            );

            let mut response = Response::new();
            for letter in input.chars() {
                let reply = match letter {
                    REPLY_SUCCESS => Reply::Success,
                    REPLY_MISS => Reply::Miss,
                    REPLY_PARTIAL => Reply::Partial,
                    _ => {
                        eprintln!("illegal input: expected alphabetical characters");
                        continue 'outer;
                    }
                };
                response.push(reply);
            }

            break Ok(response);
        }
    }

    pub fn read_turn(&mut self) -> Result<Turn, ReadTurnFlags> {
        let guess = self.read_guess()?;
        let response = self.read_reply()?;
        let mut turn: Turn = Default::default();

        for (letter, &reply) in guess.chars().zip(response.iter()) {
            turn.push(GameCell { letter, reply });
        }

        Ok(turn)
    }
}
