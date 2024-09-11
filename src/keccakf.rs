use crate::params::{ROTR, ROUNDS, WIDTH_IN_WORDS};
use crate::utils::*;
use std::intrinsics::transmute_unchecked;

// b=1600, aka round_1600
//  The state is organized as an array of 5×5 lanes, each of length w∈{1,2,4,8,16,32,64}, and b=25w.
//  When implemented on a 64-bit processor, a lane of Keccak-f[1600] can be represented as a 64-bit CPU word.
pub fn round_1600(a: [u64; WIDTH_IN_WORDS], round: usize) -> [u64; WIDTH_IN_WORDS] {
    let mut a = a;
    // θ-Theta step
    // C[x] = A[x,0] xor A[x,1] xor A[x,2] xor A[x,3] xor A[x,4],   for x in 0…4
    let mut c = [0u64; 5];
    for x in 0..5 {
        c[x] = xor(
            a[x],
            xor(a[x + 5], xor(a[x + 2 * 5], xor(a[x + 3 * 5], a[x + 4 * 5]))),
        );
    }
    // D[x] = C[x-1] xor rot(C[x+1],1),                             for x in 0…4
    let mut d = [0u64; 5];
    for x in 0..5 {
        d[x] = xor(c[(x + 4) % 5], rot(c[(x + 1) % 5], 1));
    }
    // A[x,y] = A[x,y] xor D[x],                           for (x,y) in (0…4,0…4)
    for x in 0..5 {
        for y in 0..5 {
            a[x + y * 5] = xor(a[x + y * 5], d[x]);
        }
    }

    // ρ and π steps
    // B[y,2*x+3*y] = rot(A[x,y], r[x,y]),                 for (x,y) in (0…4,0…4)
    let mut b = [0u64; WIDTH_IN_WORDS];
    for x in 0..5 {
        for y in 0..5 {
            b[y + ((2 * x + 3 * y) % 5) * 5] = rot(a[x + y * 5], ROTR[x + y * 5]);
        }
    }

    // χ-Chi step,
    // A[x,y] = B[x,y] xor ((not B[x+1,y]) and B[x+2,y]),  for (x,y) in (0…4,0…4)
    for x in 0..5 {
        for y in 0..5 {
            a[x + y * 5] = xor(
                b[x + y * 5],
                and(not(b[(x + 1) % 5 + y * 5]), b[(x + 2) % 5 + y * 5]),
            );
        }
    }

    // ι-Iota step
    // A[0,0] = A[0,0] xor RC
    let rc_u64 = crate::params::RC[round];

    a[0] = xor(a[0], rc_u64);
    return a;
}

pub fn keccakf(input: [u64; WIDTH_IN_WORDS]) -> [u64; WIDTH_IN_WORDS] {
    let mut a = input;

    for i in 0..ROUNDS {
        a = round_1600(a, i);
    }
    a
}

#[cfg(test)]
mod test {
    use crate::keccakf::round_1600;

    #[test]
    fn test_permutation_test() {
        let inputs = [4_u64; 25];
        let output = round_1600(inputs, 1);
        println!("output: {:?}", output);

        println!("aaa: {:?}", 0x80 ^ 0);
        println!("aaa: {:?}", 0 ^ 0x80);
    }
}
