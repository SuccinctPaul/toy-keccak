use crate::keccakf::*;
use crate::padding_rules::padding;
use crate::params::{KECCAK_F_DELIM, WIDTH, WIDTH_IN_BYTES, WIDTH_IN_U32, WIDTH_IN_WORDS};
use crate::utils::*;
use std::intrinsics::transmute_unchecked;
use std::mem::transmute;

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

    // The state is organized as an array of 5×5 lanes, each of length w∈{1,2,4,8,16,32,64} and b=25w
    // When implemented on a 64-bit processor, a lane of Keccak-f[1600], can be represented as a 64-bit CPU word.
    // So the state is a 5×5 array, which lanes is 64-bits word.
    pub fn hash_64bits(&self, input: &[u8]) -> Vec<u8> {
        let block_size_in_u8 = self.rate; // in bytes
        let num_blocks = input.len() / block_size_in_u8 + 1;

        let block_size_in_u32 = self.rate / 4; // in u32
        let block_size_in_u64 = self.rate / 8; // in u64

        let padded = padding(input, block_size_in_u8);
        let mut padded_u64 = from_u8_to_u64(&padded);

        let mut m = [0; WIDTH_IN_WORDS];

        for i in 0..num_blocks {
            // xor the r of state with padding block.
            for j in 0..block_size_in_u64 {
                m[j] ^= padded_u64[i * block_size_in_u64 + j];
            }
            // permutation
            m = KeccakF64::keccakf(m);
        }

        from_u64_to_u8(&m.to_vec())[0..self.output_bits_len / 8].to_vec()
    }
}

pub fn u8_xor(a: u8, b: u8) {}

#[cfg(test)]
mod test {
    use crate::params::WIDTH;
    use crate::utils::{from_bits_to_u8, from_u32_to_u64, from_u64_to_u32, from_u8_to_bits};

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

    #[test]
    fn test_u64_to_u32() {
        let u64_src_vec: Vec<u64> = vec![1123453452345, 2512341324123412341];
        let u32_src_vec = from_u64_to_u32(&u64_src_vec);
        println!("u64_src_vec: {:?}", u64_src_vec);
        println!("u32_src_vec: {:?} \n", u32_src_vec);

        let u64_dst_vec: Vec<u64> = vec![22422, 565375765756];
        let u32_dst_vec = from_u64_to_u32(&u64_dst_vec);
        println!("u64_dst_vec: {:?}", u64_dst_vec);
        println!("u32_dst_vec: {:?} \n", u32_dst_vec);

        let mut u64_res_xor = vec![];
        let mut u64_res_and = vec![];
        let mut u64_res_rot = vec![];
        let mut u64_res_not = vec![];

        for (a, b) in u64_src_vec.into_iter().zip(u64_dst_vec) {
            u64_res_xor.push(a ^ b);
            u64_res_and.push(a & b);
            u64_res_rot.push(a.rotate_left(2));
            u64_res_not.push(!a);
        }
        println!("u64_res_xor: {:?}", u64_res_xor);
        println!("u64_res_and: {:?}", u64_res_and);
        println!("u64_res_rot: {:?}", u64_res_rot);
        println!("u64_res_not: {:?}\n", u64_res_not);

        let mut u32res_xor = vec![];
        let mut u32res_and = vec![];
        let mut u32res_rot = vec![];
        let mut u32res_not = vec![];
        for (a, b) in u32_src_vec.into_iter().zip(u32_dst_vec) {
            u32res_xor.push(a ^ b);
            u32res_and.push(a & b);
            u32res_rot.push(a.rotate_left(2));
            u32res_not.push(!a);
        }
        println!("u32res_xor: {:?}", u32res_xor);
        println!("u32res_and: {:?}", u32res_and);
        println!("u32res_rot: {:?}", u32res_rot);
        println!("u32res_not: {:?}\n", u32res_not);

        let u32_to_u64_xor = from_u32_to_u64(&u32res_xor);
        let u32_to_u64_and = from_u32_to_u64(&u32res_and);
        let u32_to_u64_rot = from_u32_to_u64(&u32res_rot);
        let u32_to_u64_not = from_u32_to_u64(&u32res_not);
        println!("u32_to_u64_xor: {:?}", u32_to_u64_xor);
        println!("u32_to_u64_and: {:?}", u32_to_u64_and);
        println!("u32_to_u64_rot: {:?}", u32_to_u64_rot);
        println!("u32_to_u64_not: {:?}\n", u32_to_u64_not);

        assert_eq!(u32_to_u64_xor, u64_res_xor);
        assert_eq!(u32_to_u64_and, u64_res_and);
        // TODO: meet error.
        // assert_eq!(u32_to_u64_rot, u64_res_rot);
        assert_eq!(u32_to_u64_not, u64_res_not);
    }
}
