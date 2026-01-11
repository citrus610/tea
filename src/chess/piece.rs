use crate::chess::color::Color;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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
        match value {
            0 => Self::Pawn,
            1 => Self::Knight,
            2 => Self::Bishop,
            3 => Self::Rook,
            4 => Self::Queen,
            5 => Self::King,
            _ => panic!("invalid index")
        }
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
        match value {
            0 => Self::WhitePawn,
            1 => Self::BlackPawn,
            2 => Self::WhiteKnight,
            3 => Self::BlackKnight,
            4 => Self::WhiteBishop,
            5 => Self::BlackBishop,
            6 => Self::WhiteRook,
            7 => Self::BlackRook,
            8 => Self::WhiteQueen,
            9 => Self::BlackQueen,
            10 => Self::WhiteKing,
            11 => Self::BlackKing,
            _ => panic!("invalid index!")
        }
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