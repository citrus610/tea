use crate::chess::color::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
        debug_assert!(value < Self::COUNT as u8);

        unsafe { std::mem::transmute(value) }
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