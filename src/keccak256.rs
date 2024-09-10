use crate::keccakf::*;
use crate::padding_rules::padding;
use crate::params::{KECCAK_F_DELIM, WIDTH};
use crate::utils::*;

#[derive(Clone)]
pub struct Keccak {
    // The block length (in bytes) of Sponge Construct.
    rate: usize,
    // output_bits_num
    output_bits_len: usize,
}

impl Keccak {
    const DELIM: u8 = 0x01;

    pub fn v256() -> Keccak {
        Keccak::new(256)
    }
    pub fn v512() -> Keccak {
        Keccak::new(512)
    }
    pub fn new(output_bits_len: usize) -> Self {
        Self {
            rate: bits_to_rate(output_bits_len),

            output_bits_len,
        }
    }

    pub fn keccak256(&self, input: &[u8]) -> Vec<u8> {
        let block_size = self.rate; // in bytes
        let num_blocks = input.len() / block_size + 1;

        let mut padded = padding(input, block_size);

        let padded_bits = padded
            .iter()
            .flat_map(|x| u8_to_bits(*x))
            .collect::<Vec<_>>();

        let mut m = vec![false; WIDTH];

        for i in 0..num_blocks {
            for j in 0..block_size * 8 {
                m[j] ^= padded_bits[i * block_size * 8 + j];
            }
            m = keccakf(m);
        }
        let z = m[0..self.output_bits_len]
            .chunks(8)
            .map(|x| from_bits_to_u8(x))
            .collect::<Vec<_>>();
        z
    }
}

pub fn keccak256(input: &[u8]) -> Vec<u8> {
    let keccak = Keccak::v256();
    keccak.keccak256(input)
}
