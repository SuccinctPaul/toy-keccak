#[cfg(test)]
mod tests {
    use super::*;
    use crate::keccak256::{keccak256};
    use crate::keccakf::*;
    use crate::utils::*;
    use rand::Rng;
    use tiny_keccak::{Hasher, Keccak};

    fn expected_keccak(input: &[u8]) -> String {
        let mut hasher = Keccak::v256();
        hasher.update(&input);
        let mut hash = [0u8; 32];
        hasher.finalize(&mut hash);
        let expected = hex::encode(hash);
        expected
    }

    fn random_bytes_vec<R: Rng>(lenght: usize, rng: &mut R) -> Vec<u8> {
        let rand_vec = (0..lenght).map(|_| rng.gen()).collect::<Vec<u8>>();
        rand_vec
    }

    #[test]
    fn test_keccak256_without_tiny_keccak() {
        let rng = &mut rand::thread_rng();
        for length in [1, 4, 136, 272, 1000, 20000] {
            let input = random_bytes_vec(length, rng);
            let keccak = crate::keccak256::Keccak::v256();
            let z = keccak.keccak256(&input);
            let hex_out = hex::encode(&z);
            assert_eq!(hex_out, expected_keccak(&input));
        }
    }
    #[test]
    fn test_keccak512_with_tiny_keccak() {
        let rng = &mut rand::thread_rng();
        for length in [1, 4, 136, 272, 1000, 20000] {
            let input = random_bytes_vec(length, rng);
            let keccak = crate::keccak256::Keccak::v256();
            let z = keccak.keccak256(&input);
            let hex_out = hex::encode(&z);
            assert_eq!(hex_out, expected_keccak(&input));
        }
    }



    #[test]
    fn test_keccakf_without_ciruit() {
        let input = "bb45f489bea73ef400b0ef4cd65dcec3565b0fd75c6eb248f1fefc84dd216650327e5a5c9b02ed7ce898f8ecb2e045cded87742a7723e7fddd9ac96c8aa70f4601000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
        let expected_output = "61060054a4f8cd82609992a7604a95c9165bc95ae016a5299dd7d400dddbea9a3069922d826066fae8aad9aac3d937d6b6db11d4e3ce7663ef4236ca2f1a97a3de6259030506c8f50dcec6588ba1e7598a5f39e74f8f858f3fc04a371d52d761cb369205487758026a035dc5edd42a6bb4f1cc84c2f5a4f7915993a7b209935c40a06104fc2d4d3e337a79a6671f69fb0b3a14ccdf72f66f59828ab0f43bedab3622aa17746d3e536b9bd39974f215916563a5ed55d944d6137ce8cf03677e57bc75e502054f51b0";

        let input_u8 = hex::decode(input).unwrap();
        let input_bits = input_u8
            .iter()
            .flat_map(|x| u8_to_bits(*x))
            .collect::<Vec<_>>();
        let output_bits = keccakf(input_bits);
        let output_hex = hex::encode(
            output_bits
                .chunks(8)
                .map(|x| from_bits_to_u8(x))
                .collect::<Vec<_>>(),
        );
        assert_eq!(output_hex, expected_output);
    }
}
