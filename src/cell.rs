use crate::engine::Reply;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct GameCell {
    pub(crate) letter: char,
    pub(crate) reply: Reply,
}
