use crate::params::{ROTR, ROUNDS, WIDTH_IN_WORDS};
use crate::utils::*;

// b=1600, aka round_1600
pub fn permutation(a: [u64; WIDTH_IN_WORDS], round: usize) -> [u64; WIDTH_IN_WORDS] {
    let mut a = a;
    // θ-Theta step
    // see more Algorithm 3 in Keecak-refrence-3.0
    let mut c = [0u64; 5];
    for x in 0..5 {
        c[x] = xor(
            a[x],
            xor(a[x + 5], xor(a[x + 2 * 5], xor(a[x + 3 * 5], a[x + 4 * 5]))),
        );
    }
    let mut d = [0u64; 5];
    for x in 0..5 {
        d[x] = xor(c[(x + 4) % 5], rot(c[(x + 1) % 5], 1));
    }
    for x in 0..5 {
        for y in 0..5 {
            a[x + y * 5] = xor(a[x + y * 5], d[x]);
        }
    }

    // Rho and pi Steps
    // ρ and π steps
    // see more Algorithm 4/5 in Keecak-refrence-3.0
    let mut b = [0u64; WIDTH_IN_WORDS];
    for x in 0..5 {
        for y in 0..5 {
            b[y + ((2 * x + 3 * y) % 5) * 5] = rot(a[x + y * 5], ROTR[x + y * 5]);
        }
    }

    // χ-Chi step,
    // see more Algorithm 2 in Keecak-refrence-3.0
    for x in 0..5 {
        for y in 0..5 {
            a[x + y * 5] = xor(
                b[x + y * 5],
                and(not(b[(x + 1) % 5 + y * 5]), b[(x + 2) % 5 + y * 5]),
            );
        }
    }

    // ι-Iota step
    a[0] = xor(a[0], crate::params::RC[round]);
    return a;
}

pub fn keccakf(input: Vec<bool>) -> Vec<bool> {
    let a = input
        .chunks(64)
        .map(|e| from_bits_to_u64(e))
        .collect::<Vec<_>>();
    let mut a = a.try_into().unwrap();
    for i in 0..ROUNDS {
        a = permutation(a, i);
    }
    return a
        .iter()
        .flat_map(|x| from_u64_to_bits(*x))
        .collect::<Vec<_>>();
}

#[cfg(test)]
mod test {
    use crate::keccakf::permutation;

    #[test]
    fn test_permutation_test() {
        let inputs = [4_u64; 25];
        let output = permutation(inputs, 1);
        println!("output: {:?}", output);

        println!("aaa: {:?}", 0x80 ^ 0);
        println!("aaa: {:?}", 0 ^ 0x80);
    }
}
