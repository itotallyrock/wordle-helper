use crate::cell::GameCell;
use crate::engine::Reply;
use crate::turn::Turn;
use crate::{MAX_GUESSES, NUM_LETTERS};
use arrayvec::ArrayVec;
use log::trace;
use std::iter::{Rev, Take};
use std::slice::Iter;

#[derive(Debug)]
pub struct HardModeWordPicker {
    remaining_words: Vec<String>,
    discovered_letters: ArrayVec<Turn, MAX_GUESSES>,
}

impl HardModeWordPicker {
    pub fn new<S: AsRef<str>, D: IntoIterator<Item = S>>(dictionary: D) -> Self {
        let remaining_words = Vec::from_iter(
            dictionary
                .into_iter()
                .filter(|w| {
                    w.as_ref().len() == NUM_LETTERS
                        && w.as_ref().chars().all(|c| c.is_ascii_alphabetic())
                })
                .map(|s| s.as_ref().to_ascii_lowercase()),
        );

        let mut words = Self {
            remaining_words,
            discovered_letters: Default::default(),
        };

        // Sort words with most unique characters towards end
        words
            .remaining_words
            .sort_by_cached_key(|word| unique_letters_per_word(word));

        words
    }
    pub fn remaining(&self) -> usize {
        self.remaining_words.len()
    }
    pub fn take_turn(&mut self, turn: Turn) {
        let Turn(guess_cells) = turn;
        for (index, &GameCell { reply, letter }) in guess_cells.iter().enumerate() {
            match reply {
                Reply::Success => {
                    self.remove_words_without_letter_in_position(letter, index);
                }
                Reply::Partial => {
                    self.remove_words_not_containing(letter);
                    // TODO: Maybe do some extra logic here ff we already know this letter exists in the word or something...
                }
                Reply::Miss => {
                    let matchable_cells = [
                        GameCell {
                            letter,
                            reply: Reply::Success,
                        },
                        GameCell {
                            letter,
                            reply: Reply::Partial,
                        },
                    ];
                    // If we missed its possible this letter is elsewhere in this guess.
                    // That could be earlier or later in this guess (iterate over letters) to check for partial/successes matching this letter.
                    // If so, we can still remove this letter in this position;
                    // Otherwise, we missed completely and no words contain this letter.
                    // TODO: Make sure this check doesnt need to factor in the index of the success/partial
                    if matchable_cells
                        .iter()
                        .any(|matchable| guess_cells.contains(matchable))
                    {
                        // This letter shows up elsewhere in this word as a partial/hit so we can only remove it in this position
                        self.remove_words_with_letter_in_position(letter, index);
                    } else {
                        // This letter is a full miss and we can remove all words containing it
                        self.remove_words_containing(letter);
                    }
                }
            }
        }
    }
    pub fn remove_words_containing(&mut self, illegal_letter: char) {
        trace!("removing {} from all positions", illegal_letter);
        debug_assert!(
            illegal_letter.is_ascii_alphabetic(),
            "letter must be legal ascii letter"
        );
        debug_assert!(illegal_letter.is_lowercase(), "letter must be lowercase");

        self.remaining_words
            .retain(|word| !word.contains(illegal_letter));
        trace!(
            "has {} remaining words {:?}",
            self.remaining_words.len(),
            self.remaining_words
        );
    }
    pub fn remove_words_without_letter_in_position(
        &mut self,
        required_letter: char,
        required_position: usize,
    ) {
        trace!(
            "removing words without a {} in {}",
            required_letter,
            required_position
        );
        debug_assert!(
            required_position < NUM_LETTERS,
            "cannot remove words with letter in position that is longer than {}",
            NUM_LETTERS
        );
        debug_assert!(
            required_letter.is_ascii_alphabetic(),
            "letter must be legal ascii alphabetical"
        );
        let required_letter = required_letter.to_ascii_lowercase();

        self.remaining_words
            .retain(|word| word.chars().nth(required_position) == Some(required_letter));
        trace!(
            "has {} remaining words {:?}",
            self.remaining_words.len(),
            self.remaining_words
        );
    }
    pub fn remove_words_with_letter_in_position(
        &mut self,
        illegal_letter: char,
        required_position: usize,
    ) {
        trace!(
            "removing words with a {} in {}",
            illegal_letter,
            required_position
        );
        debug_assert!(
            required_position < NUM_LETTERS,
            "cannot remove words with letter in position that is longer than {}",
            NUM_LETTERS
        );
        debug_assert!(
            illegal_letter.is_ascii_alphabetic(),
            "letter must be legal ascii alphabetical"
        );
        let illegal_letter = illegal_letter.to_ascii_lowercase();

        self.remaining_words
            .retain(|word| word.chars().nth(required_position) != Some(illegal_letter));
        trace!(
            "has {} remaining words {:?}",
            self.remaining_words.len(),
            self.remaining_words
        );
    }
    pub fn remove_words_not_containing(&mut self, required_letter: char) {
        trace!("removing all words not containing {}", required_letter);
        debug_assert!(
            required_letter.is_ascii_alphabetic(),
            "letter must be legal ascii letter"
        );
        debug_assert!(required_letter.is_lowercase(), "letter must be lowercase");

        self.remaining_words
            .retain(|word| word.contains(required_letter));

        trace!(
            "has {} remaining words {:?}",
            self.remaining_words.len(),
            self.remaining_words
        );
    }
    pub fn pick_best_word(&mut self) -> Option<String> {
        self.remaining_words.pop()
    }
    pub fn top_10_words(&self) -> Take<Rev<Iter<'_, String>>> {
        self.remaining_words.iter().rev().take(10)
    }
}

pub(crate) fn unique_letters_per_word(word: &str) -> usize {
    const ALPHA_LEN: usize = 26;
    let mut seen = [0u8; ALPHA_LEN];

    debug_assert_eq!(
        word.to_ascii_lowercase(),
        word,
        "cannot get unique letters for non-lowercase words"
    );

    for c in word.chars() {
        debug_assert!(
            c >= 'a' && c <= 'z',
            "illegal letter in unique_letters_per_word"
        );
        let letter_index = ((c as u8) - ('a' as u8)) as usize;
        seen[letter_index] = 1;
    }

    seen.into_iter().sum::<u8>() as usize
}
