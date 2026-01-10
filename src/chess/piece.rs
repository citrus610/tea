use crate::chess::color::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Piece {
    WhitePawn,
    BlackPawn,
    WhiteKnight,
    BlackKnight,
    WhiteBishop,
    BlackBishop,
    WhiteRook,
    BlackRook,
    WhiteQueen,
    BlackQueen,
    WhiteKing,
    BlackKing
}

impl PieceKind {
    pub const COUNT: usize = 6;

    #[inline(always)]
    pub const fn index(self) -> usize {
        self as usize
    }

    #[inline(always)]
    pub const fn value(self) -> u8 {
        self as u8
    }

    #[inline(always)]
    pub const fn from_raw(value: u8) -> Self {
        debug_assert!(value < Self::COUNT as u8);

        unsafe { std::mem::transmute(value) }
    }
}

impl std::fmt::Display for PieceKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pawn => write!(f, "p"),
            Self::Knight => write!(f, "n"),
            Self::Bishop => write!(f, "b"),
            Self::Rook => write!(f, "r"),
            Self::Queen => write!(f, "q"),
            Self::King => write!(f, "k")
        }
    }
}

impl Piece {
    pub const COUNT: usize = 12;

    #[inline(always)]
    pub const fn index(self) -> usize {
        self as usize
    }

    #[inline(always)]
    pub const fn value(self) -> u8 {
        self as u8
    }

    #[inline(always)]
    pub const fn from_raw(value: u8) -> Self {
        debug_assert!(value < Self::COUNT as u8);

        unsafe { std::mem::transmute(value) }
    }

    #[inline(always)]
    pub const fn new(kind: PieceKind, color: Color) -> Self {
        Self::from_raw(kind.value() * 2 + color.value())
    }

    #[inline(always)]
    pub const fn kind(self) -> PieceKind {
        PieceKind::from_raw(self.value() / 2)
    }

    #[inline(always)]
    pub const fn color(self) -> Color {
        Color::from_raw(self.value() % 2 != 0)
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WhitePawn => write!(f, "P"),
            Self::WhiteKnight => write!(f, "N"),
            Self::WhiteBishop => write!(f, "B"),
            Self::WhiteRook => write!(f, "R"),
            Self::WhiteQueen => write!(f, "Q"),
            Self::WhiteKing => write!(f, "K"),
            Self::BlackPawn => write!(f, "p"),
            Self::BlackKnight => write!(f, "n"),
            Self::BlackBishop => write!(f, "b"),
            Self::BlackRook => write!(f, "r"),
            Self::BlackQueen => write!(f, "q"),
            Self::BlackKing => write!(f, "k")
        }
    }
}