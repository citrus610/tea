use arrayvec::ArrayVec;

use crate::chess::moves::Move;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct MoveEntry {
    pub score: i32,
    pub mv: Move
}

#[derive(Debug, Clone)]
pub struct MoveList {
    data: ArrayVec<MoveEntry, 256>,
}

impl MoveList {
    pub fn new() -> Self {
        Self { data: ArrayVec::new() }
    }

    pub fn push(&mut self, mv: Move) {
        self.data.push(MoveEntry { score: 0, mv: mv });
    }

    pub fn pop(&mut self) -> Option<MoveEntry> {
        self.data.pop()
    }

    pub fn iter_moves(&self) -> impl Iterator<Item = &Move> {
        self.data.iter().map(|e| &e.mv)
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}

impl std::ops::Deref for MoveList {
    type Target = [MoveEntry];

    fn deref(&self) -> &[MoveEntry] {
        &self.data
    }
}

impl std::ops::DerefMut for MoveList {
    fn deref_mut(&mut self) -> &mut [MoveEntry] {
        &mut self.data
    }
}