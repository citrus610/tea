use crate::chess::{movelist::MoveList, moves::Move};

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd)]
pub enum Stage {
    Hasher,
    NoisyGen,
    Noisy,
    QuietGen,
    Quiet
}

pub struct MovePicker {
    list: MoveList,
    index: usize,
    hasher: Move,
    skip: bool
}