mod bn128;
mod ecrecover;
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

    //secp256r1::test_all();
    bn128::test_all();
    //sha256::test_all();
    ecrecover::test_all();
    modexp::test_all();
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
