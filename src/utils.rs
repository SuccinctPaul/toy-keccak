use std::char::from_u32;
use std::ops::{BitAnd, BitXor, Not};

// Calculate the block length(in bytes) of Sponge Construct.
// Aka equal: (1600 - 2*bits)/8, which means b=r+c, r=2*bits.
pub fn bits_to_rate(bits: usize) -> usize {
    200 - bits / 4
}

pub fn xor<T: BitXor<Output = T>>(x: T, y: T) -> T {
    return x ^ y;
}

pub fn rot(x: u64, i: usize) -> u64 {
    return x.rotate_left(i as u32);
}

pub fn and<T: BitAnd<Output = T>>(x: T, y: T) -> T {
    x & y
}

pub fn not<T: Not<Output = T>>(x: T) -> T {
    return !x;
}

pub fn from_bits_to_u8(bools: &[bool]) -> u8 {
    assert_eq!(bools.len(), 8);
    let mut result: u8 = 0;
    let mut shift = 0;
    for &bit in bools {
        if bit {
            result |= 1 << shift;
        }
        shift += 1;
        if shift == 8 {
            break;
        }
    }
    result
}

pub fn from_bits_to_u64(bools: &[bool]) -> u64 {
    let mut result: u64 = 0;
    let mut shift = 0;
    for &bit in bools {
        if bit {
            result |= 1 << shift;
        }
        shift += 1;
        if shift == 64 {
            break;
        }
    }
    result
}

pub fn from_u64_to_bits(num: u64) -> Vec<bool> {
    let mut result = Vec::with_capacity(64);
    let mut n = num;
    for _ in 0..64 {
        result.push(n & 1 == 1);
        n >>= 1;
    }
    result
}

pub fn from_u8_to_bits(num: u8) -> Vec<bool> {
    let mut result = Vec::with_capacity(8);
    let mut n = num;
    for _ in 0..8 {
        result.push(n & 1 == 1);
        n >>= 1;
    }
    result
}

pub fn from_u8_to_u32(bytes: &Vec<u8>) -> Vec<u32> {
    bytes
        .chunks_exact(4)
        .map(|chunk| u32::from_le_bytes(chunk.try_into().unwrap()))
        .collect()
}
pub fn from_u32_to_u8(words: &Vec<u32>) -> Vec<u8> {
    words
        .into_iter()
        .flat_map(|word| word.to_le_bytes().into_iter())
        .collect()
}

pub fn from_u64_to_u8(words: &Vec<u64>) -> Vec<u8> {
    words
        .into_iter()
        .flat_map(|word| word.to_le_bytes().into_iter())
        .collect()
}

pub fn from_u8_to_u64(bytes: &[u8]) -> Vec<u64> {
    bytes
        .chunks_exact(8)
        .map(|chunk| u64::from_le_bytes(chunk.try_into().unwrap()))
        .collect()
}

pub fn from_u64_to_u32(words: &Vec<u64>) -> Vec<u32> {
    from_u8_to_u32(&from_u64_to_u8(&words))
}

pub fn from_u32_to_u64(words: &Vec<u32>) -> Vec<u64> {
    from_u8_to_u64(&from_u32_to_u8(&words))
}
