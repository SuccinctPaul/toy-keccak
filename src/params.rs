// Keccak-f(b) is an iterated permutation.

pub const ROTR: [usize; 25] = [
    0, 1, 62, 28, 27, 36, 44, 6, 55, 20, 3, 10, 43, 25, 39, 41, 45, 15, 21, 8, 18, 2, 61, 56, 14,
];

// b=25*2^\ell, generally, b=1600
// It's in num of bits.
pub const WIDTH: usize = 1600;

// WIDTH/8
pub const WIDTH_IN_BYTES: usize = 200;

// WIDTH/8, Word=64
pub const WIDTH_IN_WORDS: usize = 25;

// WIDTH/4
pub const WIDTH_IN_U32: usize = 50;

// Permutation Rounds: $n_r=12+2\ell$
pub const ROUNDS: usize = 24;

// Round Constants
pub const RC: [u64; ROUNDS] = [
    1u64,
    0x8082u64,
    0x800000000000808au64,
    0x8000000080008000u64,
    0x808bu64,
    0x80000001u64,
    0x8000000080008081u64,
    0x8000000000008009u64,
    0x8au64,
    0x88u64,
    0x80008009u64,
    0x8000000au64,
    0x8000808bu64,
    0x800000000000008bu64,
    0x8000000000008089u64,
    0x8000000000008003u64,
    0x8000000000008002u64,
    0x8000000000000080u64,
    0x800au64,
    0x800000008000000au64,
    0x8000000080008081u64,
    0x8000000000008080u64,
    0x80000001u64,
    0x8000000080008008u64,
];

// Which used is padding pad10*1
pub const KECCAK_F_DELIM: u8 = 0x01;
// The block size of sponge construct in bytes.
pub const KEKKAC_RATE: usize = 136;
