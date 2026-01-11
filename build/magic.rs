const RANKS: [u64; 8] = [
    0x00000000000000ff,
    0x000000000000ff00,
    0x0000000000ff0000,
    0x00000000ff000000,
    0x000000ff00000000,
    0x0000ff0000000000,
    0x00ff000000000000,
    0xff00000000000000
];

const FILES: [u64; 8] = [
    0x0101010101010101,
    0x0202020202020202,
    0x0404040404040404,
    0x0808080808080808,
    0x1010101010101010,
    0x2020202020202020,
    0x4040404040404040,
    0x8080808080808080
];

const RANK_18: u64 = RANKS[0] | RANKS[7];
const FILE_AB: u64 = FILES[0] | FILES[7];

pub const BISHOP_DELTA: [(i32, i32); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
pub const ROOK_DELTA: [(i32, i32); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

const BISHOP_MAGICS: [u64; 64] = [
    0xffedf9fd7cfcffff, 0xfc0962854a77f576, 0x5822022042000000, 0x2ca804a100200020,
    0x0204042200000900, 0x2002121024000002, 0xfc0a66c64a7ef576, 0x7ffdfdfcbd79ffff,
    0xfc0846a64a34fff6, 0xfc087a874a3cf7f6, 0x1001080204002100, 0x1810080489021800,
    0x0062040420010a00, 0x5028043004300020, 0xfc0864ae59b4ff76, 0x3c0860af4b35ff76,
    0x73c01af56cf4cffb, 0x41a01cfad64aaffc, 0x040c0422080a0598, 0x4228020082004050,
    0x0200800400e00100, 0x020b001230021040, 0x7c0c028f5b34ff76, 0xfc0a028e5ab4df76,
    0x0020208050a42180, 0x001004804b280200, 0x2048020024040010, 0x0102c04004010200,
    0x020408204c002010, 0x02411100020080c1, 0x102a008084042100, 0x0941030000a09846,
    0x0244100800400200, 0x4000901010080696, 0x0000280404180020, 0x0800042008240100,
    0x0220008400088020, 0x04020182000904c9, 0x0023010400020600, 0x0041040020110302,
    0xdcefd9b54bfcc09f, 0xf95ffa765afd602b, 0x1401210240484800, 0x0022244208010080,
    0x1105040104000210, 0x2040088800c40081, 0x43ff9a5cf4ca0c01, 0x4bffcd8e7c587601,
    0xfc0ff2865334f576, 0xfc0bf6ce5924f576, 0x80000b0401040402, 0x0020004821880a00,
    0x8200002022440100, 0x0009431801010068, 0xc3ffb7dc36ca8c89, 0xc3ff8a54f4ca2c89,
    0xfffffcfcfd79edff, 0xfc0863fccb147576, 0x040c000022013020, 0x2000104000420600,
    0x0400000260142410, 0x0800633408100500, 0xfc087e8e4bb2f736, 0x43ff9e4ef4ca2c89
];

const ROOK_MAGICS: [u64; 64] = [
    0xa180022080400230, 0x0040100040022000, 0x0080088020001002, 0x0080080280841000,
    0x4200042010460008, 0x04800a0003040080, 0x0400110082041008, 0x008000a041000880,
    0x10138001a080c010, 0x0000804008200480, 0x00010011012000c0, 0x0022004128102200,
    0x000200081201200c, 0x202a001048460004, 0x0081000100420004, 0x4000800380004500,
    0x0000208002904001, 0x0090004040026008, 0x0208808010002001, 0x2002020020704940,
    0x8048010008110005, 0x6820808004002200, 0x0a80040008023011, 0x00b1460000811044,
    0x4204400080008ea0, 0xb002400180200184, 0x2020200080100380, 0x0010080080100080,
    0x2204080080800400, 0x0000a40080360080, 0x02040604002810b1, 0x008c218600004104,
    0x8180004000402000, 0x488c402000401001, 0x4018a00080801004, 0x1230002105001008,
    0x8904800800800400, 0x0042000c42003810, 0x008408110400b012, 0x0018086182000401,
    0x2240088020c28000, 0x001001201040c004, 0x0a02008010420020, 0x0010003009010060,
    0x0004008008008014, 0x0080020004008080, 0x0282020001008080, 0x50000181204a0004,
    0x48fffe99fecfaa00, 0x48fffe99fecfaa00, 0x497fffadff9c2e00, 0x613fffddffce9200,
    0xffffffe9ffe7ce00, 0xfffffff5fff3e600, 0x0010301802830400, 0x510ffff5f63c96a0,
    0xebffffb9ff9fc526, 0x61fffeddfeedaeae, 0x53bfffedffdeb1a2, 0x127fffb9ffdfb5f6,
    0x411fffddffdbf4d6, 0x0801000804000603, 0x0003ffef27eebe74, 0x7645fffecbfea79e
];

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Magic {
    pub mask: u64,
    pub magic: u64,
    pub shift: u32,
    pub offset: usize
}

impl Magic {
    pub const fn index(self, occupied: u64) -> usize {
        (((occupied & self.mask) * self.magic) >> self.shift) as usize + self.offset
    }
}

pub fn slider_attacks(square: usize, occupied: u64, delta: [(i32, i32); 4]) -> u64
{
    let mut result = 0;

    for i in 0..4 {
        let mut rank = square as i32 / 8;
        let mut file = square as i32 % 8;

        loop {
            rank += delta[i].0;
            file += delta[i].1;

            if rank < 0 || file < 0 || rank > 7 || file > 7 {
                break;
            }

            let bit = 1u64 << (file + rank * 8);

            result |= bit;

            if occupied & bit != 0 {
                break;
            }
        }
    }

    result
}

pub fn magic_table(magics: [u64; 64], delta: [(i32, i32); 4]) -> [Magic; 64] {
    let mut table = [Magic { mask: 0, magic: 0, shift: 0, offset: 0 }; 64];

    for square in 0..64 {
        let edge = (RANK_18 & !RANKS[square / 8]) | (FILE_AB & !FILES[square % 8]);

        table[square].mask = slider_attacks(square, 0, delta) & !edge;
        table[square].magic = magics[square];
        table[square].shift = 64 - table[square].mask.count_ones();

        if square < 63 {
            table[square + 1].offset = table[square].offset + (1 << table[square].mask.count_ones());
        }
    }

    table
}

pub fn bishop_magic_table() -> [Magic; 64] {
    magic_table(BISHOP_MAGICS, BISHOP_DELTA)
}

pub fn rook_magic_table() -> [Magic; 64] {
    magic_table(ROOK_MAGICS, ROOK_DELTA)
}