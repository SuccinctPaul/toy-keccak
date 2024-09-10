#[cfg(test)]
mod tests {
    use rand::Rng;
    use tiny_keccak::{Hasher, Keccak};

    fn expected_keccak256(input: &[u8]) -> String {
        let mut hasher = Keccak::v256();
        hasher.update(&input);
        let mut hash = [0u8; 32];
        hasher.finalize(&mut hash);
        let expected = hex::encode(hash);
        expected
    }

    fn expected_keccak384(input: &[u8]) -> String {
        let mut hasher = Keccak::v384();
        hasher.update(&input);
        let mut hash = [0u8; 48];
        hasher.finalize(&mut hash);
        let expected = hex::encode(hash);
        expected
    }

    fn expected_keccak512(input: &[u8]) -> String {
        let mut hasher = Keccak::v512();
        hasher.update(&input);
        let mut hash = [0u8; 64];
        hasher.finalize(&mut hash);
        let expected = hex::encode(hash);
        expected
    }

    fn random_bytes_vec<R: Rng>(lenght: usize, rng: &mut R) -> Vec<u8> {
        let rand_vec = (0..lenght).map(|_| rng.gen()).collect::<Vec<u8>>();
        rand_vec
    }

    #[test]
    fn test_keccak256_with_tiny_keccak() {
        let rng = &mut rand::thread_rng();
        for length in [1, 4, 136, 272, 1000, 20000] {
            let input = random_bytes_vec(length, rng);
            let keccak = crate::keccak::Keccak::v256();
            let z = keccak.hash(&input);
            let hex_out = hex::encode(&z);
            assert_eq!(hex_out, expected_keccak256(&input));
        }
    }
    #[test]
    fn test_keccak512_with_tiny_keccak() {
        let rng = &mut rand::thread_rng();
        for length in [1, 4, 136, 272, 1000, 20000] {
            let input = random_bytes_vec(length, rng);
            let keccak = crate::keccak::Keccak::v512();
            let z = keccak.hash(&input);
            let hex_out = hex::encode(&z);
            assert_eq!(hex_out, expected_keccak512(&input));
        }
    }

    #[test]
    fn test_keccak384_with_tiny_keccak() {
        let rng = &mut rand::thread_rng();
        for length in [1, 4, 136, 272, 1000, 20000] {
            let input = random_bytes_vec(length, rng);
            let keccak = crate::keccak::Keccak::v384();
            let z = keccak.hash(&input);
            let hex_out = hex::encode(&z);
            assert_eq!(hex_out, expected_keccak384(&input));
        }
    }
}
