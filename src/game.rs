use arrayvec::{ArrayString, ArrayVec};

/// Word length for guessing and dictionary
/// Effectively how many slots are there for letters to be guessed in.
pub const NUM_LETTERS: usize = 5;
/// Maximum numbers of guesses in a game
/// After this many guesses the dictionary will reset.
pub const MAX_GUESSES: usize = 6;

/// The submitted word attempt
/// A special stack allocated string only holding 5 characters
pub type Guess = ArrayString<NUM_LETTERS>;

/// The answer given immediately after submitting a guess
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Reply {
    /// Letter matches exact position in solution (Green)
    Success,
    /// Letter does not exist in this solution (Gray)
    ///
    /// If a repeated letter, a missed Letter can still be a part of solution.
    Miss,
    /// Letter exists somewhere in the solution but not here (Yellow)
    /// Can also mean we already know where this letter belongs (from a previous `Success`) but we didnt guess with it in the correct slot (still narrows potential repeats)
    Partial,
}

/// The answers given for a submitted attempt
pub type Response = ArrayVec<Reply, NUM_LETTERS>;

/// A single cell (letter) after a guess has been replied to
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct GameCell {
    pub letter: char,
    pub reply: Reply,
}

/// The guess and solution zipped by slot
pub type Turn = ArrayVec<GameCell, NUM_LETTERS>;
