use crate::chess::{piece::PieceKind, square::Square};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum MoveKind {
    Normal,
    Castling,
    Enpassant,
    PromotionKnight,
    PromotionBishop,
    PromotionRook,
    PromotionQueen
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Move {
    data: u16
}

impl MoveKind {
    pub const COUNT: usize = 8;

    #[inline(always)]
    pub const fn index(self) -> usize {
        self as usize
    }

    #[inline(always)]
    pub const fn value(self) -> u16 {
        self as u16
    }

    #[inline(always)]
    pub const fn from_raw(value: u16) -> Self {
        debug_assert!(value < Self::COUNT as u16);

        unsafe { std::mem::transmute(value) }
    }

    #[inline(always)]
    pub fn is_special(self) -> bool {
        self != Self::Normal
    }
}

impl Move {
    pub const NULL: Self = Self { data: 0 };

    #[inline(always)]
    pub const fn new(from: Square, to: Square, kind: MoveKind) -> Self {
        Self { data: from as u16 | ((to as u16) << 6) | (kind.value() << 12) }
    }

    #[inline(always)]
    pub const fn from(self) -> Square {
        Square::from_raw(self.data as u8 % 64)
    }

    #[inline(always)]
    pub const fn to(self) -> Square {
        Square::from_raw((self.data >> 6) as u8 % 64)
    }

    #[inline(always)]
    pub const fn kind(self) -> MoveKind {
        MoveKind::from_raw(self.data >> 12)
    }

    #[inline(always)]
    pub const fn promotion_kind(self) -> Option<PieceKind> {
        match self.kind() {
            MoveKind::PromotionKnight => Some(PieceKind::Knight),
            MoveKind::PromotionBishop => Some(PieceKind::Bishop),
            MoveKind::PromotionRook => Some(PieceKind::Rook),
            MoveKind::PromotionQueen => Some(PieceKind::Queen),
            _ => None
        }
    }

    #[inline(always)]
    pub const fn is_some(self) -> bool {
        self.data != 0
    }

    #[inline(always)]
    pub const fn is_null(self) -> bool {
        self.data == 0
    }

    #[inline(always)]
    pub fn is_special(self) -> bool {
        self.kind().is_special()
    }

    #[inline(always)]
    pub fn is_castling(self) -> bool {
        self.kind() == MoveKind::Castling
    }

    #[inline(always)]
    pub fn is_enpassant(self) -> bool {
        self.kind() == MoveKind::Enpassant
    }

    #[inline(always)]
    pub fn is_promotion(self) -> bool {
        self.kind().value() >= MoveKind::PromotionKnight.value()
    }
}