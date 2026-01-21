use crate::chess::{attack::{bishop_attacks, king_attacks, knight_attacks, pawn_attacks, rook_attacks}, bitboard::Bitboard, castle::{Castle, CastleKind}, color::Color, direction::Direction, file::File, moves::Move, piece::{Piece, PieceKind}, rank::Rank, square::Square, zobrist::ZOBRIST};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FenParseError {
    MissingBoard,
    MissingColor,
    MissingCastle,
    InvalidBoard,
    InvalidColor,
    InvalidCastle
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Key {
    pub main: u64
}

#[derive(Debug, Clone)]
pub struct State {
    pieces: [Bitboard; PieceKind::COUNT],
    colors: [Bitboard; Color::COUNT],
    mailbox: [Option<Piece>; Square::COUNT],
    castles: Castle,
    enpassant: Option<Square>,
    halfmove: u8,
    checkers: Bitboard,
    blockers: [Bitboard; Color::COUNT],
    keys: Key
}

#[derive(Debug, Clone)]
pub struct Board {
    pub state: State,
    pub color: Color,
    pub stack: Vec<State>
}

impl State {
    pub const fn new() -> Self {
        Self {
            pieces: [Bitboard::new(); PieceKind::COUNT],
            colors: [Bitboard::new(); Color::COUNT],
            mailbox: [None; Square::COUNT],
            castles: Castle::new(),
            enpassant: None,
            halfmove: 0,
            checkers: Bitboard::new(),
            blockers: [Bitboard::new(); Color::COUNT],
            keys: Key { main: 0 }
        }
    }

    #[inline(always)]
    pub fn pieces(&self, kind: PieceKind) -> Bitboard {
        self.pieces[kind]
    }

    #[inline(always)]
    pub fn colors(&self, color: Color) -> Bitboard {
        self.colors[color]
    }

    #[inline(always)]
    pub fn at(&self, square: Square) -> Option<Piece> {
        self.mailbox[square]
    }

    #[inline(always)]
    pub fn occupied(&self) -> Bitboard {
        self.colors[Color::White] | self.colors[Color::Black]
    }

    #[inline(always)]
    pub const fn castles(&self) -> Castle {
        self.castles
    }

    #[inline(always)]
    pub const fn enpassant(&self) -> Option<Square> {
        self.enpassant
    }

    #[inline(always)]
    pub const fn halfmove(&self) -> u8 {
        self.halfmove
    }

    #[inline(always)]
    pub const fn checkers(&self) -> Bitboard {
        self.checkers
    }

    #[inline(always)]
    pub fn blockers(&self, color: Color) -> Bitboard {
        self.blockers[color]
    }

    #[inline(always)]
    pub const fn key(&self) -> Key {
        self.keys
    }

    #[inline(always)]
    pub fn king_square(&self, color: Color) -> Square {
        (self.pieces(PieceKind::King) & self.colors(color)).lsb()
    }

    pub fn attackers(&self, square: Square, occupied: Bitboard) -> Bitboard {
        let mut attackers = Bitboard::new();

        attackers |= pawn_attacks(square, Color::White) & self.colors(Color::Black) & self.pieces(PieceKind::Pawn);
        attackers |= pawn_attacks(square, Color::Black) & self.colors(Color::White) & self.pieces(PieceKind::Pawn);
        attackers |= king_attacks(square) & self.pieces(PieceKind::King);
        attackers |= knight_attacks(square) & self.pieces(PieceKind::Knight);
        attackers |= bishop_attacks(square, occupied) & (self.pieces(PieceKind::Bishop) | self.pieces(PieceKind::Queen));
        attackers |= rook_attacks(square, occupied) & (self.pieces(PieceKind::Rook) | self.pieces(PieceKind::Queen));

        attackers
    }

    pub fn place(&mut self, square: Square, piece: Piece) {
        self.pieces[piece.kind()].set(square);
        self.colors[piece.color()].set(square);
        self.mailbox[square] = Some(piece);
        self.keys.main ^= ZOBRIST.piece[piece][square];
    }

    pub fn remove(&mut self, square: Square) {
        if let Some(piece) = self.mailbox[square] {
            self.pieces[piece.kind()].clear(square);
            self.colors[piece.color()].clear(square);
            self.mailbox[square] = None;
            self.keys.main ^= ZOBRIST.piece[piece][square];
        }
    }

    pub fn is_attacked(&self, square: Square, color: Color, occupied: Bitboard) -> bool {
        (pawn_attacks(square, color) & self.pieces(PieceKind::Pawn) & self.colors(!color)).is_some() ||
        (king_attacks(square) & self.pieces(PieceKind::King) & self.colors(!color)).is_some() ||
        (knight_attacks(square) & self.pieces(PieceKind::Knight) & self.colors(!color)).is_some() ||
        (bishop_attacks(square, occupied) & (self.pieces(PieceKind::Bishop) | self.pieces(PieceKind::Queen)) & self.colors(!color)).is_some() ||
        (rook_attacks(square, occupied) & (self.pieces(PieceKind::Rook) | self.pieces(PieceKind::Queen)) & self.colors(!color)).is_some()
    }
}

impl Board {
    pub const FEN_STARTPOS: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    pub const fn new() -> Self {
        Self {
            state: State::new(),
            color: Color::White,
            stack: Vec::new()
        }
    }

    pub fn startpos() -> Result<Self, FenParseError> {
        Self::from_fen(Self::FEN_STARTPOS)
    }

    pub fn from_fen(fen: &str) -> Result<Self, FenParseError> {
        let mut board = Self::new();
        let mut parts = fen.split(' ');

        // Parse board
        let rows = parts.next().ok_or(FenParseError::MissingBoard)?.split('/');

        for (rank, row) in rows.rev().enumerate() {
            if rank >= Rank::COUNT {
                return Err(FenParseError::InvalidBoard);
            }

            let mut file = 0;

            for c in row.chars() {
                if file >= File::COUNT {
                    return Err(FenParseError::InvalidBoard);
                }

                if let Some(digit) = c.to_digit(10) {
                    file += digit as usize;
                    continue;
                }

                let square = Square::new(Rank::from_raw(rank as u8), File::from_raw(file as u8));
                let piece = Piece::from_char(c).ok_or(FenParseError::InvalidBoard)?;

                board.state.place(square, piece);

                file += 1;
            }
        }

        // Parse color
        board.color = match parts.next().ok_or(FenParseError::MissingColor) {
            Ok("w") => Color::White,
            Ok("b") => Color::Black,
            _ => return Err(FenParseError::InvalidColor),
        };

        // Parse castling right
        let castles = parts.next().ok_or(FenParseError::MissingCastle)?;

        if castles.len() > 4 || castles.chars().all(|c| !"KQkq-".contains(c)) {
            return Err(FenParseError::InvalidCastle);
        }

        if !castles.contains('K') {
            board.state.castles.update(CastleKind::WhiteShort);
        }

        if !castles.contains('Q') {
            board.state.castles.update(CastleKind::WhiteLong);
        }

        if !castles.contains('k') {
            board.state.castles.update(CastleKind::BlackShort);
        }

        if !castles.contains('q') {
            board.state.castles.update(CastleKind::BlackLong);
        }

        board.state.keys.main ^= ZOBRIST.castling[board.state.castles];

        // Parse enpassant square
        board.state.enpassant = match parts.next() {
            Some(enpassant) => Square::from_str(enpassant),
            None => None
        };

        // Parse halfmove counter
        board.state.halfmove = parts.next().unwrap_or_default().parse().unwrap_or(0);

        // Update threats
        board.update_threats();

        Ok(board)
    }

    pub fn make(&mut self, mv: Move) {
        debug_assert!(self.is_pseudo_legal(mv));
        debug_assert!(self.is_legal(mv));

        // Push stack
        self.stack.push(self.state.clone());

        // Move data
        let from = mv.from();
        let to = mv.to();
        let moving = self.state.at(from).unwrap();

        // Remove enpassant square
        if let Some(enpassant) = self.state.enpassant {
            self.state.keys.main ^= ZOBRIST.enpassant[enpassant.file()];
            self.state.enpassant = None;
        }

        // Update halfmove
        self.state.halfmove += 1;

        // Check capture
        if self.state.at(to).is_some() {
            self.state.halfmove = 0;
            self.state.remove(to);
        }

        // Pawn move
        if moving.kind() == PieceKind::Pawn {
            self.state.halfmove = 0;

            if from.rank().distance(to.rank()) == 2 {
                self.state.enpassant = Some(to.enpassant());
                self.state.keys.main ^= ZOBRIST.enpassant[to.file()];
            }
        }

        // Update castling right
        self.state.keys.main ^= ZOBRIST.castling[self.state.castles];

        if moving.kind() == PieceKind::King {
            if self.color == Color::White {
                self.state.castles.update(CastleKind::WhiteShort);
                self.state.castles.update(CastleKind::WhiteLong);
            }
            else {
                self.state.castles.update(CastleKind::BlackShort);
                self.state.castles.update(CastleKind::BlackLong);
            }
        }

        if let Some(castle) = CastleKind::from_corner(from) {
            self.state.castles.update(castle);
        }

        if let Some(castle) = CastleKind::from_corner(to) {
            self.state.castles.update(castle);
        }

        self.state.keys.main ^= ZOBRIST.castling[self.state.castles];

        // Move piece
        self.state.remove(from);
        self.state.place(
            to,
            match mv.promotion_kind() {
                Some(promotion_kind) => Piece::new(promotion_kind, self.color),
                _ => moving
            }
        );

        if mv.is_castling() {
            let castle = CastleKind::new(self.color, to > from);

            self.state.remove(castle.rook_from());
            self.state.place(castle.rook_to(), Piece::new(PieceKind::Rook, self.color));
        }

        if mv.is_enpassant() {
            self.state.remove(to.enpassant());
        }

        // Flip side to move
        self.color = !self.color;
        self.state.keys.main ^= ZOBRIST.color;

        // Update threat
        self.update_threats();
    }

    pub fn unmake(&mut self) {
        self.state = self.stack.pop().expect("can't unmake!");
        self.color = !self.color;
    }

    pub fn update_threats(&mut self) {
        self.state.checkers = self.state.attackers(self.state.king_square(self.color), self.state.occupied()) & self.state.colors(!self.color);

        for color in [Color::White, Color::Black] {
            self.state.blockers[color] = Bitboard::new();

            let king_square = self.state.king_square(color);
            let bishops = self.state.pieces(PieceKind::Bishop) | self.state.pieces(PieceKind::Queen);
            let rooks = self.state.pieces(PieceKind::Rook) | self.state.pieces(PieceKind::Queen);

            let mut snipers = Bitboard::new();

            snipers |= bishop_attacks(king_square, Bitboard::new()) & bishops;
            snipers |= rook_attacks(king_square, Bitboard::new()) & rooks;
            snipers &= self.state.colors(!color);

            let occupied = self.state.occupied() ^ snipers;

            for square in snipers {
                let ray = Bitboard::from_between(king_square, square) & occupied;

                if ray.is_only() {
                    self.state.blockers[color] |= ray;
                }
            }
        }
    }

    #[inline(always)]
    pub fn is_quiet(&self, mv: Move) -> bool {
        !self.is_noisy(mv)
    }

    #[inline(always)]
    pub fn is_noisy(&self, mv: Move) -> bool {
        self.state.at(mv.to()).is_some() || mv.is_promotion() || mv.is_enpassant()
    }

    pub fn is_pseudo_legal(&self, mv: Move) -> bool {
        if mv.is_null() {
            return false;
        }

        let occupied = self.state.occupied();
        let from = mv.from();
        let to = mv.to();

        let moving = match self.state.at(from) {
            Some(piece) => piece,
            None => return false
        };

        if self.state.colors(self.color).is_set(to) {
            return false;
        }
        
        if self.state.checkers().is_many() {
            return mv.is_normal() && moving.kind() == PieceKind::King && king_attacks(from).is_set(to);
        }

        if mv.is_castling() {
            let castle = match to {
                Square::G1 => CastleKind::WhiteShort,
                Square::C1 => CastleKind::WhiteLong,
                Square::G8 => CastleKind::BlackShort,
                Square::C8 => CastleKind::BlackLong,
                _ => return false
            };

            if self.state.checkers().is_some() {
                return false;
            }

            if !self.state.castles().is_allowed(castle) {
                return false;
            }

            if (Bitboard::from_between(from, to) & occupied).is_some() {
                return false;
            }

            return true;
        }

        if mv.is_enpassant() {
            if moving.kind() != PieceKind::Pawn {
                return false;
            }

            if !pawn_attacks(from, self.color).is_set(to) {
                return false;
            }

            match self.state.enpassant() {
                Some(square) => return to == square,
                None => return false
            }
        }

        if mv.is_promotion() && moving.kind() != PieceKind::Pawn {
            return false;
        }

        if moving.kind() == PieceKind::King {
            return king_attacks(from).is_set(to);
        }

        if self.state.checkers().is_some() {
            let king_square = self.state.king_square(self.color);
            let checker_square = self.state.checkers().lsb();
            let betweens = Bitboard::from_between(king_square, checker_square) | self.state.checkers();

            if !betweens.is_set(to) {
                return false;
            }
        }

        if moving.kind() == PieceKind::Pawn {
            let north = match self.color {
                Color::White => Direction::North,
                Color::Black => Direction::South
            };

            let push_mask = match self.color {
                Color::White => Bitboard::from_rank(Rank::Third),
                Color::Black => Bitboard::from_rank(Rank::Sixth)
            };

            let push = Bitboard::from_square(from).shift(north) & !occupied;
            let double_push = (push & push_mask).shift(north) & !occupied;
            let capture = pawn_attacks(from, self.color) & self.state.colors(!self.color);

            let mut span = push | double_push | capture;

            if mv.is_promotion() {
                span &= Bitboard::from_rank(Rank::First) | Bitboard::from_rank(Rank::Eighth);
            }

            return span.is_set(to);
        }

        if self.state.blockers(self.color).is_set(from) && !Bitboard::from_line(from, to).is_set(self.state.king_square(self.color)) {
            return false;
        }

        let attacks = match moving.kind() {
            PieceKind::Knight => knight_attacks(from),
            PieceKind::Bishop => bishop_attacks(from, occupied),
            PieceKind::Rook => rook_attacks(from, occupied),
            PieceKind::Queen => bishop_attacks(from, occupied) | rook_attacks(from, occupied),
            _ => Bitboard::new()
        };

        return attacks.is_set(to);
    }

    pub fn is_legal(&self, mv: Move) -> bool {
        debug_assert!(mv.is_some());

        let from = mv.from();
        let to = mv.to();
        let kind = self.state.at(from).unwrap().kind();

        if kind != PieceKind::King && kind != PieceKind::Pawn {
            return true;
        }

        if mv.is_castling() {
            let castle = CastleKind::new(self.color, to > from);

            return
                !self.state.is_attacked(castle.king_to(), self.color, self.state.occupied()) &&
                !self.state.is_attacked(castle.rook_to(), self.color, self.state.occupied());
        }

        if kind == PieceKind::King {
            return !self.state.is_attacked(to, self.color, self.state.occupied() ^ Bitboard::from_square(from));
        }

        if mv.is_enpassant() {
            if self.state.enpassant().is_some() {
                let mut occupied = self.state.occupied();

                occupied.clear(from);
                occupied.set(to);

                occupied ^= Bitboard::from_square(to).shift(
                    match self.color {
                        Color::White => Direction::South,
                        Color::Black => Direction::North
                    }
                );

                let king_square = self.state.king_square(self.color);
                let bishops = self.state.pieces(PieceKind::Bishop) | self.state.pieces(PieceKind::Queen);
                let rooks = self.state.pieces(PieceKind::Rook) | self.state.pieces(PieceKind::Queen);

                return
                    (bishop_attacks(king_square, occupied) & self.state.colors(!self.color) & bishops).is_empty() &&
                    (rook_attacks(king_square, occupied) & self.state.colors(!self.color) & rooks).is_empty();
            }
            else {
                return false;
            }
        }

        if !self.state.blockers(self.color).is_set(from) {
            return true;
        }

        if (Bitboard::from_line(from, to) & self.state.pieces(PieceKind::King) & self.state.colors(self.color)).is_some() {
            return true;
        }

        false
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rank in Rank::all().rev() {
            for file in File::all() {
                if let Some(piece) = self.state.at(Square::new(rank, file)) {
                    write!(f, "{} ", piece)?;
                }
                else {
                    write!(f, ". ")?;
                }
            }

            write!(f, "\n")?;
        }

        writeln!(f, "color: {}", self.color)?;

        Ok(())
    }
}