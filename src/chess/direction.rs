#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest
}

impl Direction {
    #[inline(always)]
    pub const fn offset(self) -> (i8, i8) {
        match self {
            Self::North => (0, 1),
            Self::South => (0, -1),
            Self::East => (1, 0),
            Self::West => (-1, 0),
            Self::NorthEast => (1, 1),
            Self::NorthWest => (-1, 1),
            Self::SouthEast => (1, -1),
            Self::SouthWest => (-1, -1)
        }
    }
}

impl std::ops::Not for Direction {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
            Self::NorthEast => Self::SouthWest,
            Self::NorthWest => Self::SouthEast,
            Self::SouthEast => Self::NorthWest,
            Self::SouthWest => Self::NorthEast,
        }
    }
}