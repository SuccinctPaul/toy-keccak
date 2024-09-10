use crate::keccakf::*;
use crate::padding_rules::padding;
use crate::params::{KECCAK_F_DELIM, WIDTH, WIDTH_IN_BYTES};
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
    pub fn v384() -> Keccak {
        Keccak::new(384)
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

    pub fn hash(&self, input: &[u8]) -> Vec<u8> {
        let block_size = self.rate; // in bytes
        let num_blocks = input.len() / block_size + 1;

        let mut padded = padding(input, block_size);

        let padded_bits = padded
            .iter()
            .flat_map(|x| from_u8_to_bits(*x))
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

pub fn u8_xor(a: u8, b: u8) {}

#[cfg(test)]
mod test {
    use crate::params::WIDTH;
    use crate::utils::{from_bits_to_u8, from_u8_to_bits};

    #[test]
    fn test_u8_xor() {
        let a: u8 = 2;
        let b: u8 = 6;

        let a_bools = from_u8_to_bits(a);
        let b_bools = from_u8_to_bits(b);

        assert_eq!(a_bools.len(), b_bools.len());

        let mut bool_xor = a_bools;
        for i in 0..bool_xor.len() {
            bool_xor[i] ^= b_bools[i];
        }
        println!("expect: {:?}", bool_xor);

        let mut actual = a ^ b;
        let actual = from_u8_to_bits(actual);
        println!("actual: {:?}", actual);
        assert_eq!(actual, bool_xor);
    }

    #[test]
    fn test_u8_vec_xor() {
        let mut m = vec![false; WIDTH];

        let target_bool = [false; 16];
        let src_bool = [true; 16];

        let mut target_bytes = target_bool
            .chunks(8)
            .map(|x| from_bits_to_u8(x))
            .collect::<Vec<_>>();
        let src_bytes = src_bool
            .chunks(8)
            .map(|x| from_bits_to_u8(x))
            .collect::<Vec<_>>();

        let mut bool_xor = target_bool;
        for i in 0..16 {
            bool_xor[i] ^= src_bool[i];
        }

        let expect = bool_xor
            .chunks(8)
            .map(|x| from_bits_to_u8(x))
            .collect::<Vec<_>>();

        let mut actual_res = target_bytes;
        // let mut dst_ptr = actual_res.as_mut_ptr();
        // let mut src_ptr = src_bytes.as_ptr();
        // for _ in 0..actual_res.len() {
        //     unsafe {
        //         *dst_ptr ^= *src_ptr;
        //         src_ptr = src_ptr.offset(1);
        //         dst_ptr = dst_ptr.offset(1);
        //     }
        // }
        for i in 0..actual_res.len() {
            actual_res[i] ^= src_bytes[i];
        }

        assert_eq!(expect, actual_res);
    }
}
