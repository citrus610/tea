use crate::chess::{piece::PieceKind, square::Square};

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd)]
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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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
        match value {
            0 => Self::Normal,
            1 => Self::Castling,
            2 => Self::Enpassant,
            3 => Self::PromotionKnight,
            4 => Self::PromotionBishop,
            5 => Self::PromotionRook,
            6 => Self::PromotionQueen,
            _ => panic!("invalid index!")
        }
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
        self.kind() >= MoveKind::PromotionKnight
    }
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.from(), self.to())?;

        if let Some(promotion_kind) = self.promotion_kind() {
            write!(f, "{}", promotion_kind)?;
        }

        Ok(())
    }
}