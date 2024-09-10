// Calculate the block length of Sponge Construct.
pub fn bits_to_rate(bits: usize) -> usize {
    200 - bits / 4
}

pub fn xor(x: u64, y: u64) -> u64 {
    return x ^ y;
}

pub fn rot(x: u64, i: usize) -> u64 {
    return x.rotate_left(i as u32);
}

pub fn and(x: u64, y: u64) -> u64 {
    return x & y;
}

pub fn not(x: u64) -> u64 {
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

pub fn from_bits(bools: &[bool]) -> u64 {
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

pub fn u64_to_bits(num: u64) -> Vec<bool> {
    let mut result = Vec::with_capacity(64);
    let mut n = num;
    for _ in 0..64 {
        result.push(n & 1 == 1);
        n >>= 1;
    }
    result
}

pub fn u8_to_bits(num: u8) -> Vec<bool> {
    let mut result = Vec::with_capacity(8);
    let mut n = num;
    for _ in 0..8 {
        result.push(n & 1 == 1);
        n >>= 1;
    }
    result
}
