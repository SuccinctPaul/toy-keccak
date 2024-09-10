use crate::params::KECCAK_F_DELIM;

pub fn padding(input: &[u8], block_size: usize) -> Vec<u8> {
    let num_blocks = input.len() / block_size + 1;

    let mut padded = vec![0u8; block_size * num_blocks];
    for i in 0..input.len() {
        padded[i] = input[i];
    }
    padded[input.len()] = KECCAK_F_DELIM;
    let last_index = padded.len() - 1;
    padded[last_index] ^= 0x80;
    padded
}
