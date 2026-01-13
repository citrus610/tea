#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub const COUNT: usize = 2;

    #[inline(always)]
    pub const fn index(self) -> usize {
        self as usize
    }

    #[inline(always)]
    pub const fn value(self) -> u8 {
        self as u8
    }

    #[inline(always)]
    pub const fn from_raw(value: bool) -> Self {
        match value {
            true => Self::Black,
            false => Self::White
        }
    }

    #[inline(always)]
    pub const fn from_char(character: char) -> Option<Self> {
        match character {
            'w' => Some(Self::White),
            'b' => Some(Self::Black),
            _ => None
        }
    }
}

impl std::ops::Not for Color {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

impl<T, const N: usize> std::ops::Index<Color> for [T; N] {
    type Output = T;

    fn index(&self, color: Color) -> &Self::Output {
        &self[color.index()]
    }
}

impl<T, const N: usize> std::ops::IndexMut<Color> for [T; N] {
    fn index_mut(&mut self, color: Color) -> &mut Self::Output {
        &mut self[color.index()]
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::White => write!(f, "w"),
            Self::Black => write!(f, "b")
        }
    }
}