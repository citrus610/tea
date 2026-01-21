#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H
}

impl File {
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
            0 => File::A,
            1 => File::B,
            2 => File::C,
            3 => File::D,
            4 => File::E,
            5 => File::F,
            6 => File::G,
            7 => File::H,
            _ => panic!("invalid index!")
        }
    }

    #[inline(always)]
    pub const fn from_char(character: char) -> Option<Self> {
        match character {
            'a' => Some(Self::A),
            'b' => Some(Self::B),
            'c' => Some(Self::C),
            'd' => Some(Self::D),
            'e' => Some(Self::E),
            'f' => Some(Self::F),
            'g' => Some(Self::G),
            'h' => Some(Self::H),
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

    pub fn all() -> impl DoubleEndedIterator<Item = Self> {
        [
            Self::A,
            Self::B,
            Self::C,
            Self::D,
            Self::E,
            Self::F,
            Self::G,
            Self::H
        ].into_iter()
    }
}

impl<T, const N: usize> std::ops::Index<File> for [T; N] {
    type Output = T;

    fn index(&self, file: File) -> &Self::Output {
        &self[file.index()]
    }
}

impl<T, const N: usize> std::ops::IndexMut<File> for [T; N] {
    fn index_mut(&mut self, file: File) -> &mut Self::Output {
        &mut self[file.index()]
    }
}

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A => write!(f, "a"),
            Self::B => write!(f, "b"),
            Self::C => write!(f, "c"),
            Self::D => write!(f, "d"),
            Self::E => write!(f, "e"),
            Self::F => write!(f, "f"),
            Self::G => write!(f, "g"),
            Self::H => write!(f, "h")
        }
    }
}