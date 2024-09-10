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

        let num_bytes = bytes.len();
        println!("inputs_bytes: {:?}", bytes);
        println!("inputs_bytes len: {:?}", bytes.len());
        println!("hex_in len: {:?}", hex_in.len());

        let mut stack_1 = bytes;
        stack_1.reverse();
        println!("stack_1: {:?}", stack_1);

        // push_reverse_to_alt
        let stack_2 = (1..num_bytes)
            .map(|i| stack_1[num_bytes - i])
            .collect::<Vec<_>>();
        println!("stack_2: {:?}", stack_2);

        let mut stack_3 = stack_1;
        stack_3.push(0x80);
        for i in 0..59 {
            stack_3.push(0);
        }
        // push u32
        println!("0");
        // push u32
        println!("59*8: {:X}", 59 * 8);
    }
}
