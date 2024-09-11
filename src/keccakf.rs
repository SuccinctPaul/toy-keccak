//！Keccak-f[b] permutations:
//！ - ℓ:  0  1   2   3   4   5    6
//！ - w:  1  2   4   8  16  32   64 (2ˡ)
//！ - b: 25 50 100 200 400 800 1600 (25 × 2ˡ)
//！Keccak256 specifies Keccak-f[1600] only, hence ℓ=6, w=64, b=1600.

use crate::params::{ROTR, ROUNDS, WIDTH_IN_BYTES, WIDTH_IN_U32, WIDTH_IN_WORDS};
use crate::utils::*;
use std::intrinsics::transmute_unchecked;

pub struct KeccakF64;

impl KeccakF64 {
    pub type STATE = [u64; WIDTH_IN_WORDS];
    // Theta step
    pub fn theta(a: Self::STATE) -> Self::STATE {
        // C[x] = A[x,0] xor A[x,1] xor A[x,2] xor A[x,3] xor A[x,4],   for x in 0…4
        let mut c = [0u64; 5];
        for x in 0..5 {
            c[x] = a[x] ^ a[x + 5] ^ a[x + 2 * 5] ^ a[x + 3 * 5] ^ a[x + 4 * 5];
        }
        // D[x] = C[x-1] xor rot(C[x+1],1),                             for x in 0…4
        let mut d = [0u64; 5];
        for x in 0..5 {
            d[x] = xor(c[(x + 4) % 5], rot(c[(x + 1) % 5], 1));
        }

        // A[x,y] = A[x,y] xor D[x],                           for (x,y) in (0…4,0…4)
        let mut res = [0u64; WIDTH_IN_WORDS];
        for x in 0..5 {
            for y in 0..5 {
                res[x + y * 5] = xor(a[x + y * 5], d[x]);
            }
        }
        res
    }

    // ρ and π steps
    // B[y,2*x+3*y] = rot(A[x,y], r[x,y]),                 for (x,y) in (0…4,0…4)
    pub fn rho_phi(a: Self::STATE) -> Self::STATE {
        let mut res = [0u64; WIDTH_IN_WORDS];
        for x in 0..5 {
            for y in 0..5 {
                res[y + ((2 * x + 3 * y) % 5) * 5] = rot(a[x + y * 5], ROTR[x + y * 5]);
            }
        }
        res
    }

    // χ-Chi step,
    // A[x,y] = B[x,y] xor ((not B[x+1,y]) and B[x+2,y]),  for (x,y) in (0…4,0…4)
    pub fn chi(b: Self::STATE) -> Self::STATE {
        let mut res = [0u64; WIDTH_IN_WORDS];
        for x in 0..5 {
            for y in 0..5 {
                res[x + y * 5] = xor(
                    b[x + y * 5],
                    and(not(b[(x + 1) % 5 + y * 5]), b[(x + 2) % 5 + y * 5]),
                );
            }
        }
        res
    }

    // ι-Iota step
    // A[0,0] = A[0,0] xor RC
    pub fn iota(mut c: Self::STATE, round: usize) -> Self::STATE {
        c[0] = xor(c[0], crate::params::RC_64_BITS[round]);
        c
    }

    // b=1600, aka round_1600
    //  The state is organized as an array of 5×5 lanes, each of length w∈{1,2,4,8,16,32,64}, and b=25w.
    //  When implemented on a 64-bit processor, a lane of Keccak-f[1600] can be represented as a 64-bit CPU word.
    pub fn round(a: Self::STATE, round: usize) -> Self::STATE {
        // Theta step
        let a = Self::theta(a);

        // ρ and π steps
        let b = Self::rho_phi(a);

        // χ-Chi step,
        let c = Self::chi(b);

        // ι-Iota step
        Self::iota(c, round)
    }

    pub fn keccakf(input: Self::STATE) -> Self::STATE {
        let mut a = input;

        for i in 0..ROUNDS {
            a = Self::round(a, i);
        }
        a
    }
}

pub struct KeccakF32;

impl KeccakF32 {
    pub type STATE = [u32; WIDTH_IN_U32];

    // Theta step
    pub fn theta(a: Self::STATE) -> Self::STATE {
        // C[x] = A[x,0] xor A[x,1] xor A[x,2] xor A[x,3] xor A[x,4],   for x in 0…4
        let mut c = [0; 5];
        for x in 0..5 {
            c[x] = a[x] ^ a[x + 5] ^ a[x + 2 * 5] ^ a[x + 3 * 5] ^ a[x + 4 * 5];
        }
        // D[x] = C[x-1] xor rot(C[x+1],1),                             for x in 0…4
        let mut d = [0; 5];
        for x in 0..5 {
            d[x] = xor(c[(x + 4) % 5], c[(x + 1) % 5].rotate_left(1));
        }

        // A[x,y] = A[x,y] xor D[x],                           for (x,y) in (0…4,0…4)
        let mut res = [0; WIDTH_IN_U32];
        for x in 0..5 {
            for y in 0..5 {
                res[x + y * 5] = xor(a[x + y * 5], d[x]);
            }
        }
        res
    }

    // ρ and π steps
    // B[y,2*x+3*y] = rot(A[x,y], r[x,y]),                 for (x,y) in (0…4,0…4)
    pub fn rho_phi(a: Self::STATE) -> Self::STATE {
        let mut res = [0; WIDTH_IN_U32];
        for x in 0..5 {
            for y in 0..5 {
                res[y + ((2 * x + 3 * y) % 5) * 5] =
                    a[x + y * 5].rotate_left(ROTR[x + y * 5] as u32);
            }
        }
        res
    }

    // χ-Chi step,
    // A[x,y] = B[x,y] xor ((not B[x+1,y]) and B[x+2,y]),  for (x,y) in (0…4,0…4)
    pub fn chi(b: Self::STATE) -> Self::STATE {
        let mut res = [0; WIDTH_IN_U32];
        for x in 0..5 {
            for y in 0..5 {
                res[x + y * 5] = xor(
                    b[x + y * 5],
                    and(not(b[(x + 1) % 5 + y * 5]), b[(x + 2) % 5 + y * 5]),
                );
            }
        }
        res
    }

    // ι-Iota step
    // A[0,0] = A[0,0] xor RC
    pub fn iota(mut c: Self::STATE, round: usize) -> Self::STATE {
        // TODO regenerate the RC
        c[0] = xor(c[0], crate::params::RC_64_BITS[round] as u32);
        c
    }

    // b=1600, aka round_1600
    //  The state is organized as an array of 5×5 lanes, each of length w∈{1,2,4,8,16,32,64}, and b=25w.
    //  When implemented on a 64-bit processor, a lane of Keccak-f[1600] can be represented as a 64-bit CPU word.
    pub fn round(a: Self::STATE, round: usize) -> Self::STATE {
        // Theta step
        let a = Self::theta(a);

        // ρ and π steps
        let b = Self::rho_phi(a);

        // χ-Chi step,
        let c = Self::chi(b);

        // ι-Iota step
        Self::iota(c, round)
    }

    pub fn keccakf(input: Self::STATE) -> Self::STATE {
        let mut a = input;

        for i in 0..ROUNDS {
            a = Self::round(a, i);
        }
        a
    }
}

#[cfg(test)]
mod test {
    use crate::keccakf::KeccakF64;

    #[test]
    fn test_permutation_test() {
        let inputs = [4_u64; 25];
        let output = KeccakF64::round(inputs, 1);
        println!("output: {:?}", output);

        println!("aaa: {:?}", 0x80 ^ 0);
        println!("aaa: {:?}", 0 ^ 0x80);
    }
}
