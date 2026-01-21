use crate::chess::moves::Move;

pub const MAX_PLY: usize = 128;
pub const MAX_STACK: usize = MAX_PLY + 8;

#[derive(Debug, Clone, Copy)]
pub struct Pv {
    data: [Move; MAX_STACK],
    size: usize
}

impl Pv {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            data: [Move::NULL; MAX_STACK],
            size: 0
        }
    }

    #[inline(always)]
    pub const fn front(&self) -> Move {
        self.data[0]
    }

    #[inline(always)]
    pub const fn update(&mut self, mv: Move, other: &Pv) {
        self.data[0] = mv;

        let mut i = 0;

        while i < other.size {
            self.data[i + 1] = other.data[i];

            i += 1;
        }

        self.size = other.size + 1;
    }

    #[inline(always)]
    pub const fn clear(&mut self) {
        self.data = [Move::NULL; MAX_STACK]
    }
}

impl std::ops::Index<usize> for Pv {
    type Output = Move;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}