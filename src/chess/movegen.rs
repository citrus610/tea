use crate::chess::{attack::*, bitboard::Bitboard, board::Board, castle::CastleKind, color::Color, direction::Direction, movelist::MoveList, moves::{Move, MoveKind}, piece::PieceKind, rank::Rank, square::Square};

struct White;
struct Black;

struct Quiet;
struct Noisy;
struct All;

trait ColorTrait {
    const COLOR: Color;
}

trait KindTrait {
    const QUIET: bool;
    const NOISY: bool;
}

impl ColorTrait for White {
    const COLOR: Color = Color::White;
}

impl ColorTrait for Black {
    const COLOR: Color = Color::Black;
}

impl KindTrait for Quiet {
    const QUIET: bool = true;
    const NOISY: bool = false;
}

impl KindTrait for Noisy {
    const QUIET: bool = false;
    const NOISY: bool = true;
}

impl KindTrait for All {
    const QUIET: bool = true;
    const NOISY: bool = true;
}

impl Board {
    fn push_normal_moves(list: &mut MoveList, from: Square, targets: Bitboard) {
        for to in targets {
            list.push(Move::new(from, to, MoveKind::Normal));
        }
    }

    fn push_promotion_moves(list: &mut MoveList, from: Square, to: Square) {
        list.push(Move::new(from, to, MoveKind::PromotionQueen));
        list.push(Move::new(from, to, MoveKind::PromotionRook));
        list.push(Move::new(from, to, MoveKind::PromotionBishop));
        list.push(Move::new(from, to, MoveKind::PromotionKnight));
    }

    fn push_castling_moves<C: ColorTrait>(&self, list: &mut MoveList) {
        let king_square = self.state.king_square(C::COLOR);
        let occupied = self.state.occupied();
        let castles = match C::COLOR {
            Color::White => [CastleKind::WhiteShort, CastleKind::WhiteLong],
            Color::Black => [CastleKind::BlackShort, CastleKind::BlackLong]
        };

        for castle in castles {
            if !self.state.castles().is_allowed(castle) {
                continue;
            }

            if (Bitboard::from_between(king_square, castle.rook_from()) & occupied).is_some() {
                continue;
            }

            list.push(Move::new(king_square, castle.king_to(), MoveKind::Castling));
        }
    }

    fn push_pawn_moves<C: ColorTrait, K: KindTrait>(&self, list: &mut MoveList, checkmask: Bitboard) {
        let north = match C::COLOR {
            Color::White => Direction::North,
            Color::Black => Direction::South
        };

        let north_east = match C::COLOR {
            Color::White => Direction::NorthEast,
            Color::Black => Direction::SouthWest
        };

        let north_west = match C::COLOR {
            Color::White => Direction::NorthWest,
            Color::Black => Direction::SouthEast
        };

        let mask_push = match C::COLOR {
            Color::White => Bitboard::from_rank(Rank::Third),
            Color::Black => Bitboard::from_rank(Rank::Sixth)
        };

        let mask_promotion = match C::COLOR {
            Color::White => Bitboard::from_rank(Rank::Eighth),
            Color::Black => Bitboard::from_rank(Rank::First)
        };

        let pawns = self.state.colors(C::COLOR) & self.state.pieces(PieceKind::Pawn);
        let enemy = self.state.colors(!C::COLOR);
        let empty = !self.state.occupied();

        let mut push = pawns.shift(north) & empty;
        let mut double_push = (push & mask_push).shift(north) & empty;
        let mut east = pawns.shift(north_east) & enemy & checkmask;
        let mut west = pawns.shift(north_west) & enemy & checkmask;

        push &= checkmask;
        double_push &= checkmask;

        if K::NOISY {
            let push_promotion = push & mask_promotion;
            let east_promotion = east & mask_promotion;
            let west_promotion = west & mask_promotion;

            for to in push_promotion {
                Self::push_promotion_moves(list, to.shift(!north).unwrap(), to);
            }

            for to in east_promotion {
                Self::push_promotion_moves(list, to.shift(!north_east).unwrap(), to);
            }

            for to in west_promotion {
                Self::push_promotion_moves(list, to.shift(!north_west).unwrap(), to);
            }
        }

        push &= !mask_promotion;
        east &= !mask_promotion;
        west &= !mask_promotion;

        if K::QUIET {
            for to in push {
                list.push(Move::new(to.shift(!north).unwrap(), to, MoveKind::Normal));
            }

            for to in double_push {
                list.push(Move::new(to.shift(!north).unwrap().shift(!north).unwrap(), to, MoveKind::Normal));
            }
        }

        if K::NOISY {
            for to in east {
                list.push(Move::new(to.shift(!north_east).unwrap(), to, MoveKind::Normal));
            }

            for to in west {
                list.push(Move::new(to.shift(!north_west).unwrap(), to, MoveKind::Normal));
            }

            if let Some(enpassant) = self.state.enpassant() {
                let ep = pawn_attacks(enpassant, !C::COLOR) & pawns;

                for from in ep {
                    list.push(Move::new(from, enpassant, MoveKind::Enpassant));
                }
            }
        }
    }

    fn movegen<C: ColorTrait, K: KindTrait>(&self) -> MoveList {
        let mut list = MoveList::new();

        let us = self.state.colors(C::COLOR);
        let them = self.state.colors(!C::COLOR);
        let occupied = us | them;
        let checkers = self.state.checkers();
        let blockers = self.state.blockers(C::COLOR);

        let mut movable = !us;

        if !K::QUIET {
            movable = them;
        }

        if !K::NOISY {
            movable = !occupied;
        }

        let king_square = self.state.king_square(C::COLOR);

        Self::push_normal_moves(&mut list, king_square, king_attacks(king_square) & movable);

        if checkers.is_many() {
            return list;
        }

        let checkmask = match checkers.is_some() {
            true => checkers | Bitboard::from_between(king_square, checkers.lsb()),
            false => Bitboard::from_raw(0xffffffffffffffff)
        };

        movable &= checkmask;

        if K::QUIET && checkers.is_empty() {
            self.push_castling_moves::<C>(&mut list);
        }

        self.push_pawn_moves::<C, K>(&mut list, checkmask);

        let knights = self.state.pieces(PieceKind::Knight) & self.state.colors(C::COLOR) & !blockers;

        for from in knights {
            Self::push_normal_moves(&mut list, from, knight_attacks(from) & movable);
        }

        let bishops = (self.state.pieces(PieceKind::Bishop) | self.state.pieces(PieceKind::Queen)) & self.state.colors(C::COLOR);

        for from in bishops {
            let mut targets = bishop_attacks(from, occupied) & movable;

            if (Bitboard::from_square(from) & blockers).is_some() {
                targets &= Bitboard::from_line(from, king_square);
            }

            Self::push_normal_moves(&mut list, from, targets);
        }

        let rooks = (self.state.pieces(PieceKind::Rook) | self.state.pieces(PieceKind::Queen)) & self.state.colors(C::COLOR);

        for from in rooks {
            let mut targets = rook_attacks(from, occupied) & movable;

            if (Bitboard::from_square(from) & blockers).is_some() {
                targets &= Bitboard::from_line(from, king_square);
            }

            Self::push_normal_moves(&mut list, from, targets);
        }

        list
    }

    pub fn generate_quiet_moves(&self) -> MoveList {
        match self.color {
            Color::White => self.movegen::<White, Quiet>(),
            Color::Black => self.movegen::<Black, Quiet>()
        }
    }

    pub fn generate_noisy_moves(&self) -> MoveList {
        match self.color {
            Color::White => self.movegen::<White, Noisy>(),
            Color::Black => self.movegen::<Black, Noisy>()
        }
    }

    pub fn generate_moves(&self) -> MoveList {
        match self.color {
            Color::White => self.movegen::<White, All>(),
            Color::Black => self.movegen::<Black, All>()
        }
    }
}

pub fn perft<const ROOT: bool>(board: &mut Board, depth: i32) -> usize {
    let moves = board.generate_moves();
    let mut count = 0;

    for &mv in moves.iter_moves() {
        if !board.is_legal(mv) {
            continue;
        }

        board.make(mv);

        let nodes = if depth > 1 {
            perft::<false>(board, depth - 1)
        }
        else {
            1
        };

        board.unmake();

        if ROOT {
            println!("{} - {}", mv, nodes);
        }

        count += nodes;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starpos() {
        let mut board = Board::startpos().unwrap();
        assert_eq!(perft::<false>(&mut board, 6), 119060324);
    }

    #[test]
    fn kiwipete() {
        let mut board = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();
        assert_eq!(perft::<false>(&mut board, 5), 193690690);
    }

    #[test]
    fn position_3() {
        let mut board = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
        assert_eq!(perft::<false>(&mut board, 7), 178633661);
    }

    #[test]
    fn position_4() {
        let mut board = Board::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1").unwrap();
        assert_eq!(perft::<false>(&mut board, 5), 15833292);
    }

    #[test]
    fn position_5() {
        let mut board = Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();
        assert_eq!(perft::<false>(&mut board, 5), 89941194);
    }

    #[test]
    fn position_6() {
        let mut board = Board::from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10").unwrap();
        assert_eq!(perft::<false>(&mut board, 5), 164075551);
    }
}