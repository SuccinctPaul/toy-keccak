use crate::params::{ROTR, ROUNDS, WIDTH_IN_WORDS};
use crate::utils::*;

pub fn round(a: [u64; WIDTH_IN_WORDS], round: usize) -> [u64; WIDTH_IN_WORDS] {
    let mut a = a;
    // θ-Theta step
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
    let mut b = [0u64; 25];
    for x in 0..5 {
        for y in 0..5 {
            b[y + ((2 * x + 3 * y) % 5) * 5] = rot(a[x + y * 5], ROTR[x + y * 5]);
        }
    }

    // χ-Chi step
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
    let a = input.chunks(64).map(|e| from_bits(e)).collect::<Vec<_>>();
    let mut a = a.try_into().unwrap();
    for i in 0..ROUNDS {
        a = round(a, i);
    }
    return a.iter().flat_map(|x| u64_to_bits(*x)).collect::<Vec<_>>();
}
