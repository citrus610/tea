use crate::chess::{color::Color, direction::Direction, file::File, rank::Rank};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8
}

impl Square {
    pub const COUNT: usize = 64;

    #[inline(always)]
    pub const fn new(rank: Rank, file: File) -> Self {
        Self::from_raw(rank.value() * 8 + file.value())
    }

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
        // I can do this all day
        match value {
            0 => Self::A1,
            1 => Self::B1,
            2 => Self::C1,
            3 => Self::D1,
            4 => Self::E1,
            5 => Self::F1,
            6 => Self::G1,
            7 => Self::H1,
            8 => Self::A2,
            9 => Self::B2,
            10 => Self::C2,
            11 => Self::D2,
            12 => Self::E2,
            13 => Self::F2,
            14 => Self::G2,
            15 => Self::H2,
            16 => Self::A3,
            17 => Self::B3,
            18 => Self::C3,
            19 => Self::D3,
            20 => Self::E3,
            21 => Self::F3,
            22 => Self::G3,
            23 => Self::H3,
            24 => Self::A4,
            25 => Self::B4,
            26 => Self::C4,
            27 => Self::D4,
            28 => Self::E4,
            29 => Self::F4,
            30 => Self::G4,
            31 => Self::H4,
            32 => Self::A5,
            33 => Self::B5,
            34 => Self::C5,
            35 => Self::D5,
            36 => Self::E5,
            37 => Self::F5,
            38 => Self::G5,
            39 => Self::H5,
            40 => Self::A6,
            41 => Self::B6,
            42 => Self::C6,
            43 => Self::D6,
            44 => Self::E6,
            45 => Self::F6,
            46 => Self::G6,
            47 => Self::H6,
            48 => Self::A7,
            49 => Self::B7,
            50 => Self::C7,
            51 => Self::D7,
            52 => Self::E7,
            53 => Self::F7,
            54 => Self::G7,
            55 => Self::H7,
            56 => Self::A8,
            57 => Self::B8,
            58 => Self::C8,
            59 => Self::D8,
            60 => Self::E8,
            61 => Self::F8,
            62 => Self::G8,
            63 => Self::H8,
            _ => panic!("invalid index!")
        }
    }

    #[inline(always)]
    pub const fn rank(self) -> Rank {
        Rank::from_raw(self.value() / 8)
    }

    #[inline(always)]
    pub const fn file(self) -> File {
        File::from_raw(self.value() % 8)
    }

    #[inline(always)]
    pub const fn flip_file(self) -> Self {
        Self::from_raw(self.value() ^ 7)
    }

    #[inline(always)]
    pub const fn flip_rank(self) -> Self {
        Self::from_raw(self.value() ^ 56)
    }

    #[inline(always)]
    pub const fn relative(self, color: Color) -> Self {
        match color {
            Color::White => self,
            Color::Black => self.flip_rank()
        }
    }

    #[inline(always)]
    pub const fn shift(self, direction: Direction) -> Option<Self> {
        let file_new = self.file() as i8 + direction.offset().0;
        let rank_new = self.rank() as i8 + direction.offset().1;

        if rank_new < 0 || file_new < 0 || rank_new >= Rank::COUNT as i8 || file_new >= File::COUNT as i8 {
            return None;
        }

        Some(Self::new(Rank::from_raw(rank_new as u8), File::from_raw(file_new as u8)))
    }

    #[inline(always)]
    pub fn chebyshev(self, other: Self) -> u8 {
        self.file().distance(other.file()).max(self.rank().distance(other.rank()))
    }

    pub fn all() -> impl DoubleEndedIterator<Item = Self> {
        [
            Self::A1, Self::B1, Self::C1, Self::D1, Self::E1, Self::F1, Self::G1, Self::H1,
            Self::A2, Self::B2, Self::C2, Self::D2, Self::E2, Self::F2, Self::G2, Self::H2,
            Self::A3, Self::B3, Self::C3, Self::D3, Self::E3, Self::F3, Self::G3, Self::H3,
            Self::A4, Self::B4, Self::C4, Self::D4, Self::E4, Self::F4, Self::G4, Self::H4,
            Self::A5, Self::B5, Self::C5, Self::D5, Self::E5, Self::F5, Self::G5, Self::H5,
            Self::A6, Self::B6, Self::C6, Self::D6, Self::E6, Self::F6, Self::G6, Self::H6,
            Self::A7, Self::B7, Self::C7, Self::D7, Self::E7, Self::F7, Self::G7, Self::H7,
            Self::A8, Self::B8, Self::C8, Self::D8, Self::E8, Self::F8, Self::G8, Self::H8
        ].into_iter()
    }
}

impl<T, const N: usize> std::ops::Index<Square> for [T; N] {
    type Output = T;

    fn index(&self, square: Square) -> &Self::Output {
        &self[square.index()]
    }
}

impl<T, const N: usize> std::ops::IndexMut<Square> for [T; N] {
    fn index_mut(&mut self, square: Square) -> &mut Self::Output {
        &mut self[square.index()]
    }
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.file(), self.rank())
    }
}