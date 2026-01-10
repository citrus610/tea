pub struct Zobrist {
    pub piece: [[u64; 64]; 12],
    pub enpassant: [u64; 8],
    pub castling: [u64; 16],
    pub color: u64
}

const fn rand64(mut seed: u64) -> u64 {
    seed ^= seed >> 12;
    seed ^= seed << 25;
    seed ^= seed >> 27;

    seed.wrapping_mul(2685821657736338717u64)
}

pub const ZOBRIST: Zobrist = {
    let mut zobrist = Zobrist {
        piece: [[0; 64]; 12],
        enpassant: [0; 8],
        castling: [0; 16],
        color: 0
    };

    let mut seed = rand64(1070372);

    // Piece
    let mut piece = 0;
    
    while piece < 12 {
        let mut square = 0;

        while square < 64 {
            zobrist.piece[piece][square] = seed;
            seed = rand64(seed);

            square += 1;
        }

        piece += 1;
    }

    // Enpassant
    let mut file = 0;

    while file < 8 {
        zobrist.enpassant[file] = seed;
        seed = rand64(seed);

        file += 1;
    }

    // Enpassant
    let mut castle = 0;

    while castle < 16 {
        zobrist.castling[castle] = seed;
        seed = rand64(seed);

        castle += 1;
    }

    // Color
    zobrist.color = seed;

    zobrist
};