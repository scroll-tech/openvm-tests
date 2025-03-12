extern crate core;

use std::{env, io::{self, Write}};
use std::fs::File;
use std::path::Path;
use std::str::FromStr;
use rand::{Rng, RngCore, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;
use tiny_keccak::Hasher;

fn main() {
    println!("cargo:rerun-if-env-changed=KECCAK256_TEST_VECTOR_LEN");
    println!("cargo:rerun-if-env-changed=KECCAK256_TEST_VECTOR_CASE_MIN_LEN");
    println!("cargo:rerun-if-env-changed=KECCAK256_TEST_VECTOR_CASE_MAX_LEN");
    println!("cargo:rerun-if-env-changed=KECCAK256_TEST_VECTOR_SEED");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);
    generate_keccak256_test_vector(
        out_dir,
        read_env_var_or_default("KECCAK256_TEST_VECTOR_LEN", 32),
        read_env_var_or_default("KECCAK256_TEST_VECTOR_CASE_MIN_LEN", 0),
        read_env_var_or_default("KECCAK256_TEST_VECTOR_CASE_MAX_LEN", 128),
        read_env_var_or_default("KECCAK256_TEST_VECTOR_SEED", 42),
    ).unwrap();
}

fn generate_keccak256_test_vector(
    out_dir: &Path,
    len: usize,
    case_min_len: usize,
    case_max_len: usize,
    seed: u64
) -> io::Result<()> {
    let mut file = File::create(out_dir.join("keccak256_test_vector.rs"))?;
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);

    writeln!(file, "const CASES: &[(&[u8], [u8; 32])] = &[")?;
    let mut buffer = Vec::<u8>::with_capacity(case_max_len);
    let mut hash = [0u8; 32];
    for _ in 0..len {
        let input_len = rng.random_range(case_min_len..=case_max_len);
        buffer.clear();
        buffer.resize(input_len, 0);
        rng.fill_bytes(buffer.as_mut_slice());
        let mut hasher = tiny_keccak::Keccak::v256();
        hasher.update(buffer.as_slice());
        hasher.finalize(&mut hash);
        writeln!(file, r#"    (
        &hex_literal::hex!("{}"),
        hex_literal::hex!("{}"),
    ),"#,
               hex::encode(buffer.as_slice()),
               hex::encode(&hash)
        )?;
    }
    writeln!(file, "];")?;

    Ok(())
}

fn read_env_var_or_default<T: FromStr>(var: &str, default: T) -> T {
    env::var(var).ok().and_then(|s|s.parse().ok()).unwrap_or(default)
}
