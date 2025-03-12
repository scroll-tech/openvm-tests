#[cfg(feature = "openvm")]
use openvm_keccak256_guest::keccak256;
#[cfg(feature = "openvm")]
use tiny_keccak as _;
use crate::should_eq;

include!(concat!(env!("OUT_DIR"), "/keccak256_test_vector.rs"));

pub fn test_all() {
    println!("keccak256 test:");
    for (idx, (input, expected)) in CASES.iter().enumerate() {
        let outcome = keccak256(input);
        should_eq!(outcome, *expected, "keccak256#{idx}");
    }
    println!("keccak256 test done");
}

#[cfg(not(feature = "openvm"))]
fn keccak256(input: &[u8]) -> [u8; 32] {
    use tiny_keccak::Hasher;
    let mut output = [0u8; 32];
    let mut hasher = tiny_keccak::Keccak::v256();
    hasher.update(input);
    hasher.finalize(&mut output);
    output
}
