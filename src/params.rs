// Keccak-f(b) is an iterated permutation.
// From https://keccak.team/keccak_specs_summary.html

use crate::keccakf::{KeccakF32, KeccakF64};
// b=25*2^\ell=r+b, generally, b=1600
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

// Which used is padding pad10*1
pub const KECCAK_F_DELIM: u8 = 0x01;
// The block size of sponge construct in bytes: `r = KEKKAC_RATE * 8`
// The sponge construction operates on a state of `b=r+c` bits. The value `r` is called the bitrate
// and the value `c` the capacity.
pub const KECCAK_RATE: usize = 136;

impl KeccakF64 {
    pub const ROTR: [usize; 25] = [
        0, 1, 62, 28, 27, 36, 44, 6, 55, 20, 3, 10, 43, 25, 39, 41, 45, 15, 21, 8, 18, 2, 61, 56,
        14,
    ];

    // Round Constants in 64 bits
    pub const RC_64_BITS: [u64; ROUNDS] = [
        1,
        0x8082,
        0x800000000000808a,
        0x8000000080008000,
        0x808b,
        0x80000001,
        0x8000000080008081,
        0x8000000000008009,
        0x8a,
        0x88,
        0x80008009,
        0x8000000a,
        0x8000808b,
        0x800000000000008b,
        0x8000000000008089,
        0x8000000000008003,
        0x8000000000008002,
        0x8000000000000080,
        0x800a,
        0x800000008000000a,
        0x8000000080008081,
        0x8000000000008080,
        0x80000001,
        0x8000000080008008,
    ];
}

impl KeccakF32 {
    pub const ROTR: [usize; 25] = [
        0, 1, 62, 28, 27, 36, 44, 6, 55, 20, 3, 10, 43, 25, 39, 41, 45, 15, 21, 8, 18, 2, 61, 56,
        14,
    ];

    // Round Constants in 64 bits
    pub const RC_32_BITS: [u32; ROUNDS * 2] = [
        0x00000001, 0x00000000, 0x00000000, 0x00000089, 0x00000000, 0x8000008B, 0x00000000,
        0x80008080, 0x00000001, 0x0000008B, 0x00000001, 0x00008000, 0x00000001, 0x80008088,
        0x00000001, 0x80000082, 0x00000000, 0x0000000B, 0x00000000, 0x0000000A, 0x00000001,
        0x00008082, 0x00000000, 0x00008003, 0x00000001, 0x0000808B, 0x00000001, 0x8000000B,
        0x00000001, 0x8000008A, 0x00000001, 0x80000081, 0x00000000, 0x80000081, 0x00000000,
        0x80000008, 0x00000000, 0x00000083, 0x00000000, 0x80008003, 0x00000001, 0x80008088,
        0x00000000, 0x80000088, 0x00000001, 0x00008000, 0x00000000, 0x80008082,
    ];
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::from_u64_to_u32;

    #[test]
    fn test_gen_RC_32_bits() {
        let rc_32_bits = from_u64_to_u32(&KeccakF64::RC_64_BITS.to_vec());
        for x in rc_32_bits {
            println!("{:6X}", x);
        }
    }

    // #[test]
    // fn test_gen_round_constant() {
    //     let rc = |t| { x ^ t }
    // }
}
