use crate::params::ROTR;
use crate::utils::*;

pub fn round(a: [u64; 25], rc: u64) -> [u64; 25] {
    let mut a = a;
    // θ step
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

    // ρ and π steps
    let mut b = [0u64; 25];
    for x in 0..5 {
        for y in 0..5 {
            b[y + ((2 * x + 3 * y) % 5) * 5] = rot(a[x + y * 5], ROTR[x + y * 5]);
        }
    }

    // χ step
    for x in 0..5 {
        for y in 0..5 {
            a[x + y * 5] = xor(
                b[x + y * 5],
                and(not(b[(x + 1) % 5 + y * 5]), b[(x + 2) % 5 + y * 5]),
            );
        }
    }

    // ι step
    a[0] = xor(a[0], rc);
    return a;
}

pub fn keccakf(input: Vec<bool>) -> Vec<bool> {
    let a = input.chunks(64).map(|e| from_bits(e)).collect::<Vec<_>>();
    let mut a = a.try_into().unwrap();
    for i in 0..24 {
        a = round(a, crate::params::RC[i]);
    }
    return a.iter().flat_map(|x| u64_to_bits(*x)).collect::<Vec<_>>();
}
