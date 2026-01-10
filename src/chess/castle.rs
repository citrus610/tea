use crate::chess::{color::Color, square::Square};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CastleKind {
    WhiteShort = 1,
    WhiteLong = 2,
    BlackShort = 4,
    BlackLong = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Castle {
    data: u8
}

impl CastleKind {
    #[inline(always)]
    pub const fn index(self) -> usize {
        self as usize
    }

    #[inline(always)]
    pub const fn value(self) -> u8 {
        self as u8
    }

    #[inline(always)]
    pub const fn new(color: Color, is_short: bool) -> Self {
        match color {
            Color::White => if is_short { Self::WhiteShort } else { Self::WhiteLong },
            Color::Black => if is_short { Self::BlackShort } else { Self::BlackLong }
        }
    }

    #[inline(always)]
    pub const fn king_to(self) -> Square {
        match self {
            Self::WhiteShort => Square::G1,
            Self::WhiteLong => Square::C1,
            Self::BlackShort => Square::G8,
            Self::BlackLong => Square::C8,
        }
    }

    #[inline(always)]
    pub const fn rook_to(self) -> Square {
        match self {
            Self::WhiteShort => Square::F1,
            Self::WhiteLong => Square::D1,
            Self::BlackShort => Square::F8,
            Self::BlackLong => Square::D8,
        }
    }

    pub fn all() -> impl DoubleEndedIterator<Item = Self> {
        [
            Self::WhiteShort,
            Self::WhiteLong,
            Self::BlackShort,
            Self::BlackLong
        ].into_iter()
    }
}

impl Castle {
    #[inline(always)]
    pub const fn new() -> Self {
        Self { data: 0 }
    }

    #[inline(always)]
    pub const fn index(self) -> usize {
        self.data as usize
    }

    #[inline(always)]
    pub const fn value(self) -> u8 {
        self.data as u8
    }

    #[inline(always)]
    pub fn update(&mut self, kind: CastleKind) {
        self.data |= kind.value();
    }

    #[inline(always)]
    pub const fn is_allowed(self, kind: CastleKind) -> bool {
        self.value() & kind.value() == 0
    }
}