mod bn128;
mod ecrecover;
mod keccak256;
mod modexp;
mod secp256r1;
mod sha256;


#[cfg(feature = "openvm")]
#[allow(unused_imports, clippy::single_component_path_imports)]
use {
    openvm::platform as openvm_platform,
    openvm_algebra_guest::IntMod,
    openvm_bigint_guest, // trigger extern u256 (this may be unneeded)
    openvm_ecc_guest::k256::Secp256k1Point,
    openvm_ecc_guest::p256::P256Point,
    openvm_keccak256_guest, // trigger extern native-keccak256
    openvm_pairing_guest::bn254::Bn254G1Affine,
};

#[cfg(feature = "openvm")]
openvm_algebra_guest::moduli_macros::moduli_init! {
    "0x30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47", // Bn254Fp Coordinate field
    "0x30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001", // Bn254 Scalar
    "0xFFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFE FFFFFC2F", // secp256k1 Coordinate field
    "0xFFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFE BAAEDCE6 AF48A03B BFD25E8C D0364141", // secp256k1 Scalar field
    "0xffffffff 00000001 00000000 00000000 00000000 ffffffff ffffffff ffffffff", // secp256r1_coord_prime
    "0xffffffff 00000000 ffffffff ffffffff bce6faad a7179e84 f3b9cac2 fc632551" // secp256r1_scalar_prime
}

//use openvm_ecc_guest::weierstrass::WeierstrassPoint;

#[cfg(feature = "openvm")]
openvm_ecc_guest::sw_macros::sw_init! {
    Secp256k1Point,
    P256Point,
    Bn254G1Affine
}

#[cfg(feature = "openvm")]
openvm_algebra_complex_macros::complex_init! {
    Bn254Fp2 { mod_idx = 0 },
}

#[cfg(feature = "openvm")]
openvm::entry!(main);

fn main() {
    #[cfg(feature = "openvm")]
    {
        setup_all_moduli();
        setup_all_curves();
        setup_all_complex_extensions();
    }

    let mask = read_mask();

    for (idx, (test, name)) in [
        bn128::test_alt_bn128_add,
        bn128::test_alt_bn128_mul,
        bn128::test_alt_bn128_pair,
        ecrecover::test_all,
        modexp::test_all,
        secp256r1::test_p256_verify,
        sha256::test_all,
        keccak256::test_all,
    ]
    .into_iter()
    .zip([
        "alt_bn128_add",
        "alt_bn128_mul",
        "alt_bn128_pair",
        "ecrecover",
        "modexp",
        "p256_verify",
        "sha256",
        "keccak256",
    ])
    .enumerate()
    {
        if mask & (1 << idx) == 0 {
            println!("Skipping test {name}");
            continue;
        }
        println!("Running test {name}");
        test();
        println!("Done");
    }
}

#[cfg(not(feature = "openvm"))]
fn read_mask() -> u8 {
    std::env::args()
        .skip(1)
        .next()
        .map(|mask| {
            u8::from_str_radix(&mask, 10)
                .or_else(|_| {
                    u8::from_str_radix(
                        if mask.starts_with("0x") {
                            &mask[2..]
                        } else {
                            &mask
                        },
                        16,
                    )
                })
                .unwrap()
        })
        .unwrap_or(0xff)
}

#[cfg(feature = "openvm")]
fn read_mask() -> u8 {
    openvm::io::read_vec()[0]
}

#[macro_export]
macro_rules! should_eq {
    ($left:expr, $right:expr, $($arg:tt)+) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    println!("fail: {}", format_args!($($arg)+));
                    #[cfg(feature = "strict")]
                    assert_eq!($left, $right, $($arg)+);
                } else {
                    println!("pass: {}", format_args!($($arg)+));
                }
            }
        }
    };
}

#[macro_export]
macro_rules! should_be_true {
    ($cond:expr, $($arg:tt)+) => {
        if !$cond {
            println!("fail: {}", format_args!($($arg)+));
            #[cfg(feature = "strict")]
            assert!($cond, $($arg)+);
        } else {
            println!("pass: {}", format_args!($($arg)+));
        }
    };
}
