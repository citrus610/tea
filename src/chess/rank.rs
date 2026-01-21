use crate::chess::color::Color;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
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
            0 => Rank::First,
            1 => Rank::Second,
            2 => Rank::Third,
            3 => Rank::Fourth,
            4 => Rank::Fifth,
            5 => Rank::Sixth,
            6 => Rank::Seventh,
            7 => Rank::Eighth,
            _ => panic!("invalid index!")
        }
    }

    #[inline(always)]
    pub const fn from_char(character: char) -> Option<Self> {
        match character {
            '1' => Some(Self::First),
            '2' => Some(Self::Second),
            '3' => Some(Self::Third),
            '4' => Some(Self::Fourth),
            '5' => Some(Self::Fifth),
            '6' => Some(Self::Sixth),
            '7' => Some(Self::Seventh),
            '8' => Some(Self::Eighth),
            _ => None
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