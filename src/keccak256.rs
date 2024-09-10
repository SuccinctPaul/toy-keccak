use crate::keccakf::*;
use crate::utils::*;

pub fn keccak256(input: &[u8]) -> Vec<u8> {
    let block_size = 136; // in bytes
    let num_blocks = input.len() / block_size + 1;

    let mut padded = vec![0u8; block_size * num_blocks];
    for i in 0..input.len() {
        padded[i] = input[i];
    }
    padded[input.len()] = 0x01;
    let last_index = padded.len() - 1;
    padded[last_index] ^= 0x80;

    let padded_bits = padded
        .iter()
        .flat_map(|x| u8_to_bits(*x))
        .collect::<Vec<_>>();

    let mut m = vec![false; 1600];

    for i in 0..num_blocks {
        for j in 0..block_size * 8 {
            m[j] ^= padded_bits[i * block_size * 8 + j];
        }
        m = keccakf(m);
    }
    let z = m[0..256]
        .chunks(8)
        .map(|x| from_bits_to_u8(x))
        .collect::<Vec<_>>();
    z
}

pub fn keccak256_bits(input: Vec<bool>) -> Vec<bool> {
    assert_eq!(input.len() % 8, 0); // input should be bytes.
    let block_size_in_bytes = 136; // in bytes
    let input_len_in_bytes = input.len() / 8;
    let num_blocks = input_len_in_bytes / block_size_in_bytes + 1;

    let mut padded = vec![];
    for _ in 0..block_size_in_bytes * 8 * num_blocks {
        padded.push(false);
    }

    // register input
    for i in 0..input_len_in_bytes * 8 {
        padded[i] = input[i];
    }

    // append 0x01 = 1000 0000 after the last input
    padded[input_len_in_bytes * 8] = true;

    // pad 0s
    let last_index = padded.len() - 1;
    for i in input_len_in_bytes * 8 + 1..last_index {
        padded[i] = false;
    }

    // xor 0x80 = 0000 0001 with the last byte.
    // however the last bit is ensured to be 0, so just fill 1.
    padded[last_index] = true;

    let mut m = vec![false; 1600];
    for i in 0..num_blocks {
        for j in 0..block_size_in_bytes * 8 {
            let word = j / 64;
            let bit = j % 64;
            m[word * 64 + bit] ^= padded[i * block_size_in_bytes * 8 + j];
        }
        m = keccakf(m);
    }
    m[0..256].to_vec()
}
