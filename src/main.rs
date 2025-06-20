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
    openvm_keccak256_guest, // trigger extern native-keccak256
    openvm_k256::Secp256k1Point,
    openvm_p256::P256Point,
    openvm_pairing::bn254::Bn254G1Affine,
};

#[cfg(feature = "openvm")]
openvm::entry!(main);

#[cfg(feature = "openvm")]
openvm::init!();

fn main() {
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
