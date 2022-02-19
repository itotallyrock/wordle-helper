use crate::engine::{Exit, Reply};
use crate::NUM_LETTERS;
use std::io::{stdout, Lines, StdinLock, Write};
use std::iter::Flatten;

pub const REPLY_SUCCESS: char = '+';
pub const REPLY_MISS: char = '.';
pub const REPLY_PARTIAL: char = '-';

const MAX_INPUT: usize = 256;

fn read_input(
    lines: &mut Flatten<Lines<StdinLock>>,
    input_name: &'static str,
    prompt: String,
) -> Result<String, Exit> {
    loop {
        print!("{}: ", prompt);
        stdout().lock().flush();
        let input = if let Some(input) = lines.next() {
            input
        } else {
            eprintln!("failed to read {} from stdin: please try again", input_name);
            continue;
        };

        let guess = input.trim().to_ascii_lowercase();

        match guess.as_str() {
            "exit" | "quit" | "q" => break Err(Exit),
            _ if guess.len() != NUM_LETTERS => {
                eprintln!("illegal {}: expected 5 characters", input_name);
            }
            _ => break Ok(guess),
        }
    }
}

pub fn read_guess(lines: &mut Flatten<Lines<StdinLock>>) -> Result<String, Exit> {
    loop {
        let input = read_input(lines, "input", String::from("input guess"))?;

        if input.chars().all(|c| c.is_ascii_alphabetic()) {
            break Ok(input);
        } else {
            eprintln!("illegal input: expected alphabetical characters");
        }
    }
}

pub fn read_reply(lines: &mut Flatten<Lines<StdinLock>>) -> Result<[Reply; 5], Exit> {
    'outer: loop {
        let input = read_input(
            lines,
            "reply",
            format!(
                "input reply (miss: '{}', hit: '{}' partial: '{}')",
                REPLY_MISS, REPLY_SUCCESS, REPLY_PARTIAL
            ),
        )?;
        debug_assert_eq!(
            input.len(),
            5,
            "illegal reply was not checked in read_input"
        );
        let mut response = [Reply::Miss; NUM_LETTERS];
        for (index, c) in (0..5).zip(input.chars()) {
            let reply = match c {
                REPLY_SUCCESS => Reply::Success,
                REPLY_MISS => Reply::Miss,
                REPLY_PARTIAL => Reply::Partial,
                _ => {
                    eprintln!("illegal input: expected alphabetical characters");
                    continue 'outer;
                }
            };
            response[index] = reply;
        }

        break Ok(response);
    }
}
