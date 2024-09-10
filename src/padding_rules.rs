use crate::params::KECCAK_F_DELIM;

pub fn padding(input: &[u8], block_size: usize) -> Vec<u8> {
    let num_blocks = input.len() / block_size + 1;
    let target_len = block_size * num_blocks;
    let padding_len = target_len - input.len();

    let mut padded = input.to_vec();
    padded.push(KECCAK_F_DELIM);
    for i in 2..padding_len {
        padded.push(0);
    }

    padded.push(0x80);

    padded
}

#[cfg(test)]
mod test {
    use crate::padding_rules::padding;
    use crate::params::KECCAK_F_DELIM;

    #[test]
    fn test_padding() {
        let hex_in = "6162636462636465636465666465666765666768666768696768696a68696a6b696a6b6c6a6b6c6d6b6c6d6e6c6d6e6f6d6e6f706e6f7071";
        let hex: String = hex_in
            .chars()
            .filter(|c| c.is_ascii_digit() || c.is_ascii_alphabetic())
            .collect();

        let bytes: Vec<u8> = (0..hex.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
            .collect::<Vec<u8>>();

        let padding = padding(&bytes, KECCAK_F_DELIM as usize);
        println!("padding: {:?}", padding);
    }
}
