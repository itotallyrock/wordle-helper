use crate::cell::GameCell;
use crate::NUM_LETTERS;

#[derive(Debug, Copy, Clone)]
pub struct Turn(pub(crate) [GameCell; NUM_LETTERS]);
