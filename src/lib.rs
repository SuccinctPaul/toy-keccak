#![feature(core_intrinsics)]

use crate::keccak::Keccak;

pub mod keccak;
pub mod keccakf;
pub mod padding_rules;
pub mod params;
pub mod test;
pub mod utils;

pub fn keccak256(input: &[u8]) -> Vec<u8> {
    let keccak = Keccak::v256();
    keccak.hash_64bits(input)
}
