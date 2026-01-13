include!(concat!(env!("OUT_DIR"), "/magic.rs"));

use crate::chess::bitboard::Bitboard;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Magic {
    pub mask: u64,
    pub magic: u64,
    pub shift: u32,
    pub offset: usize
}

impl Magic {
    #[inline(always)]
    pub const fn index(self, occupied: Bitboard) -> usize {
        ((occupied.value() & self.mask).wrapping_mul(self.magic) >> self.shift) as usize + self.offset
    }
}