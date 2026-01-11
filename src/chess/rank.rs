use crate::chess::color::Color;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum Rank {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth
}

impl Rank {
    pub const COUNT: usize = 8;

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
            0 => Self::First,
            1 => Self::Second,
            2 => Self::Third,
            3 => Self::Fourth,
            4 => Self::Fifth,
            5 => Self::Sixth,
            6 => Self::Seventh,
            7 => Self::Eighth,
            _ => panic!("invalid index!")
        }
    }

    #[inline(always)]
    pub const fn flip(self) -> Self {
        Self::from_raw(self.value() ^ 7)
    }

    #[inline(always)]
    pub const fn distance(self, other: Self) -> u8 {
        self.value().abs_diff(other.value())
    }

    #[inline(always)]
    pub const fn relative(self, color: Color) -> Self {
        match color {
            Color::White => self,
            Color::Black => self.flip()
        }
    }

    pub fn all() -> impl DoubleEndedIterator<Item = Self> {
        [
            Self::First,
            Self::Second,
            Self::Third,
            Self::Fourth,
            Self::Fifth,
            Self::Sixth,
            Self::Seventh,
            Self::Eighth
        ].into_iter()
    }
}

impl<T, const N: usize> std::ops::Index<Rank> for [T; N] {
    type Output = T;

    fn index(&self, rank: Rank) -> &Self::Output {
        &self[rank.index()]
    }
}

impl<T, const N: usize> std::ops::IndexMut<Rank> for [T; N] {
    fn index_mut(&mut self, rank: Rank) -> &mut Self::Output {
        &mut self[rank.index()]
    }
}

impl std::fmt::Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::First => write!(f, "1"),
            Self::Second => write!(f, "2"),
            Self::Third => write!(f, "3"),
            Self::Fourth => write!(f, "4"),
            Self::Fifth => write!(f, "5"),
            Self::Sixth => write!(f, "6"),
            Self::Seventh => write!(f, "7"),
            Self::Eighth => write!(f, "8")
        }
    }
}