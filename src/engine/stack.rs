use crate::{chess::moves::Move, engine::pv::{MAX_STACK, Pv}};

#[derive(Debug, Clone, Copy)]
pub struct StackEntry {
    pub pv: Pv,
    pub mv: Move,
    pub eval: Option<i32>
}

#[derive(Debug, Clone)]
pub struct Stack {
    data: [StackEntry; MAX_STACK]
}

impl Stack {
    pub const fn new() -> Self {
        Self { data: () }
    }
}

impl std::ops::Index<usize> for Stack {
    type Output = StackEntry;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}