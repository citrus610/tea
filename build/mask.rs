fn file(square: i32) -> i32 {
    square % 8
}

fn rank(square: i32) -> i32 {
    square / 8
}

fn square(rank: i32, file: i32) -> i32 {
    rank * 8 + file
}

pub fn between() -> Vec<[u64; 64]> {
    let mut table = vec![[0; 64]; 64];

    for a in 0..64 {
        for b in 0..64 {
            if a == b {
                continue;
            }

            let mut mask: u64 = 0;

            if rank(a) == rank(b) {
                let rank = rank(a);
                let mut i = file(a).min(file(b)) + 1;

                while i < file(a).max(file(b)) {
                    mask |= 1 << square(rank, i);
                    i += 1;
                }
            }

            if file(a) == file(b) {
                let file = file(a);
                let mut i = rank(a).min(rank(b)) + 1;

                while i < rank(a).max(rank(b)) {
                    mask |= 1 << square(i, file);
                    i += 1;
                }
            }

            if file(a).abs_diff(file(b)) == rank(a).abs_diff(rank(b)) {
                let mut r = rank(a);
                let mut f = file(a);

                let delta_r = if rank(a) < rank(b) { 1 } else { -1 };
                let delta_f = if file(a) < file(b) { 1 } else { -1 };

                loop {
                    r += delta_r;
                    f += delta_f;

                    if r == rank(b) || f == file(b) {
                        break;
                    }

                    mask |= 1 << square(r, f);
                }
            }

            table[a as usize][b as usize] = mask;
            table[b as usize][a as usize] = mask;
        }
    }

    table
}

pub fn line() -> Vec<[u64; 64]> {
    let mut table = vec![[0; 64]; 64];

    for a in 0..64 {
        for b in 0..64 {
            if a == b {
                continue;
            }

            let mut mask: u64 = 0;

            if rank(a) == rank(b) {
                mask = 0xff << (rank(a) * 8);
            }

            if file(a) == file(b) {
                mask = 0x0101010101010101 << file(a);
            }

            if file(a).abs_diff(file(b)) == rank(a).abs_diff(rank(b)) {
                mask |= 1 << a;
                mask |= 1 << b;

                let mut r = rank(a);
                let mut f = file(a);

                let delta_r = if rank(a) < rank(b) { 1 } else { -1 };
                let delta_f = if file(a) < file(b) { 1 } else { -1 };

                loop {
                    r += delta_r;
                    f += delta_f;

                    if r < 0 || f < 0 || r > 7 || f > 7 {
                        break;
                    }

                    mask |= 1 << square(r, f);
                }

                r = rank(a);
                f = file(a);

                loop {
                    r -= delta_r;
                    f -= delta_f;

                    if r < 0 || f < 0 || r > 7 || f > 7 {
                        break;
                    }

                    mask |= 1 << square(r, f);
                }
            }

            table[a as usize][b as usize] = mask;
            table[b as usize][a as usize] = mask;
        }
    }

    table
}