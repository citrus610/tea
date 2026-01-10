use crate::chess::{file::File, rank::Rank, square::Square};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Bitboard {
    data: u64
}

impl Bitboard {
    #[inline(always)]
    pub const fn new() -> Self {
        Self { data: 0 }
    }

    #[inline(always)]
    pub const fn from_square(square: Square) -> Self {
        Self { data: 1u64 << square.index() }
    }

    #[inline(always)]
    pub const fn from_rank(rank: Rank) -> Self {
        Self { data: 0xFFu64 << (rank.index() * 8) }
    }

    #[inline(always)]
    pub const fn from_file(file: File) -> Self {
        Self { data: 0x0101010101010101u64 << file.index() }
    }

    #[inline(always)]
    pub const fn value(self) -> u64 {
        self.data
    }

    #[inline(always)]
    pub const fn lsb(self) -> Square {
        Square::from_raw(self.data.trailing_zeros() as u8)
    }

    #[inline(always)]
    pub const fn count(self) -> usize {
        self.data.count_ones() as usize
    }

    #[inline(always)]
    pub fn set(&mut self, square: Square) {
        self.data |= 1u64 << square.index();
    }

    #[inline(always)]
    pub fn clear(&mut self, square: Square) {
        self.data &= !(1u64 << square.index());
    }

    #[inline(always)]
    pub const fn is_empty(self) -> bool {
        self.data == 0
    }

    #[inline(always)]
    pub const fn is_some(self) -> bool {
        self.data != 0
    }

    #[inline(always)]
    pub const fn is_only(self) -> bool {
        self.is_some() && !self.is_many()
    }

    #[inline(always)]
    pub const fn is_many(self) -> bool {
        self.data & (self.data - 1) != 0
    }

    #[inline(always)]
    pub const fn is_set(self, square: Square) -> bool {
        self.data & (1u64 << square.index()) != 0
    }
}

impl Iterator for Bitboard {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_empty() {
            None
        } else {
            let lsb = self.lsb();
            self.data &= self.data - 1;
            Some(lsb)
        }
    }
}

impl std::ops::BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self { data: self.data & rhs.data }
    }
}

impl std::ops::BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self { data: self.data | rhs.data }
    }
}

impl std::ops::BitXor for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { data: self.data ^ rhs.data }
    }
}

impl std::ops::BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.data |= rhs.data;
    }
}

impl std::ops::BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.data &= rhs.data;
    }
}

impl std::ops::Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self { data: !self.data }
    }
}