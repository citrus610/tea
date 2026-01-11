use std::{env, fs::File, io::{BufWriter, Write}, path::Path};

use crate::{attack::{BISHOP_TABLE_SIZE, ROOK_TABLE_SIZE, bishop_table, king_table, knight_table, pawn_table, rook_table}, magic::{bishop_magic_table, rook_magic_table}};

mod magic;
mod attack;

fn write_magic() {
    let out = env::var_os("OUT_DIR").unwrap();
    let path = Path::new(&out).join("magic.rs");
    let file = File::create(path).unwrap();
    let mut buffer = BufWriter::new(file);

    writeln!(buffer, "pub const {}: [Magic; 64] = {:?};\n", "BISHOP_MAGICS", bishop_magic_table()).unwrap();
    writeln!(buffer, "pub const {}: [Magic; 64] = {:?};\n", "ROOK_MAGICS", rook_magic_table()).unwrap();
}

fn write_attack() {
    let out = env::var_os("OUT_DIR").unwrap();
    let path = Path::new(&out).join("attack.rs");
    let file = File::create(path).unwrap();
    let mut buffer = BufWriter::new(file);

    writeln!(buffer, "const {}: [[u64; 64]; 2] = {:?};", "PAWN_ATTACKS", pawn_table()).unwrap();
    writeln!(buffer, "const {}: [u64; 64] = {:?};", "KING_ATTACKS", king_table()).unwrap();
    writeln!(buffer, "const {}: [u64; 64] = {:?};", "KNIGHT_ATTACKS", knight_table()).unwrap();

    write!(buffer, "const {}: [u64; {}] = [", "BISHOP_ATTACKS", BISHOP_TABLE_SIZE).unwrap();

    for attacks in bishop_table() {
        write!(buffer, "{}, ", attacks).unwrap();
    }

    write!(buffer, "];\n").unwrap();

    write!(buffer, "const {}: [u64; {}] = [", "ROOK_ATTACKS", ROOK_TABLE_SIZE).unwrap();

    for attacks in rook_table() {
        write!(buffer, "{}, ", attacks).unwrap();
    }

    write!(buffer, "];\n").unwrap();
}

fn main() {
    write_magic();
    write_attack();

    println!("cargo::rerun-if-changed=build/magic.rs");
    println!("cargo::rerun-if-changed=build/attack.rs");
    println!("cargo::rerun-if-changed=build/build.rs");
}