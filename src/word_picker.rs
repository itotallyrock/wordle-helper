use std::iter::{Rev, Take};
use std::slice::Iter;

use log::trace;

use crate::game::GameCell;
use crate::game::Reply;
use crate::game::Turn;
use crate::NUM_LETTERS;

#[derive(Debug)]
pub struct HardModeWordPicker {
    remaining_words: Vec<String>,
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

        let mut words = Self { remaining_words };

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
        for (index, &GameCell { reply, letter }) in turn.iter().enumerate() {
            match reply {
                Reply::Success => {
                    // If we get a letter in the correct spot remove all words that dont
                    self.remove_words_without_letter_in_position(letter, index);
                }
                Reply::Partial => {
                    // If we get a partial match we know the word must contain this letter
                    self.remove_words_not_containing(letter);
                    // We also know this position isn't correct for this letter
                    self.remove_words_with_letter_in_position(letter, index);
                    // TODO: Maybe do some extra logic here ff we already know this letter exists in the word or something...
                }
                Reply::Miss => {
                    // Check if this is a matched repeated letter; meaning, if there is another occurrence that is a success/partial
                    let (prior_cells, post_cells) = turn.split_at(index);
                    let has_matching_repeat = prior_cells
                        .iter()
                        .chain(post_cells.iter().skip(1))
                        .any(|game_cell| {
                            const REPEATED_MATCHING_REPLIES: [Reply; 2] =
                                [Reply::Partial, Reply::Success];
                            let &GameCell {
                                letter: potential_repeat,
                                reply: repeat_reply,
                            } = game_cell;

                            potential_repeat == letter
                                && REPEATED_MATCHING_REPLIES.contains(&repeat_reply)
                        });

                    // If we have a matched repeat, we can only remove this letter in this position because there could be a second letter so we can't remove them all.
                    if has_matching_repeat {
                        self.remove_words_with_letter_in_position(letter, index);
                    } else {
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
            ('a'..='z').contains(&c),
            "illegal letter in unique_letters_per_word"
        );
        let letter_index = ((c as u8) - b'a') as usize;
        seen[letter_index] = 1;
    }

    seen.into_iter().sum::<u8>() as usize
}
