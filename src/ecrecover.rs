use hex_literal::hex;
use revm_precompile::primitives::address;
use revm_precompile::secp256k1::ec_recover_run;
use revm_precompile::{Address, Bytes};

const CASES: &[(&str, &[u8], Option<Address>)] = &[
    (
        "ethereum/tests/CALLCODEEcrecover0",
        &hex!("18c547e4f7b0f325ad1e56f57e26c745b09a3e503d86e00e5255ff7f715d3d1c000000000000000000000000000000000000000000000000000000000000001c73b1693892219d736caba55bdb67216e485557ea6b6af75f37096c9aa6a5a75feeb940b1d03b21e36b0e47e79769f095fe2ab855bd91e3a38756b7d75a9c4549"),
        Some(address!("a94f5374fce5edbc8e2a8697c15331677e6ebf0b")),
    ),
    (
        "ethereum/tests/CALLCODEEcrecover3",
        &hex!("2f380a2dea7e778d81affc2443403b8fe4644db442ae4862ff5bb3732829cdb9000000000000000000000000000000000000000000000000000000000000001b6b65ccb0558806e9b097f27a396d08f964e37b8b7af6ceeb516ff86739fbea0a37cbc8d883e129a4b1ef9d5f1df53c4f21a3ef147cf2a50a4ede0eb06ce092d4"),
        Some(address!("e4319f4b631c6d0fcfc84045dbcb676865fe5e13"))
    ),
    (
        "ethereum/tests/CALLCODEEcrecoverH_prefixed0",
        &hex!("00c547e4f7b0f325ad1e56f57e26c745b09a3e503d86e00e5255ff7f715d3d1c000000000000000000000000000000000000000000000000000000000000001c73b1693892219d736caba55bdb67216e485557ea6b6af75f37096c9aa6a5a75feeb940b1d03b21e36b0e47e79769f095fe2ab855bd91e3a38756b7d75a9c4549"),
        Some(address!("a0b29af6a56d6cfef6415cb195ccbe540e006d0a"))
    ),
    (
        "ethereum/tests/CALLCODEEcrecoverR_prefixed0",
        &hex!("18c547e4f7b0f325ad1e56f57e26c745b09a3e503d86e00e5255ff7f715d3d1c000000000000000000000000000000000000000000000000000000000000001c00b1693892219d736caba55bdb67216e485557ea6b6af75f37096c9aa6a5a75feeb940b1d03b21e36b0e47e79769f095fe2ab855bd91e3a38756b7d75a9c4549"),
        None,
    ),
    (
        "ethereum/tests/CALLCODEEcrecoverS_prefixed0",
        &hex!("18c547e4f7b0f325ad1e56f57e26c745b09a3e503d86e00e5255ff7f715d3d1c000000000000000000000000000000000000000000000000000000000000001c73b1693892219d736caba55bdb67216e485557ea6b6af75f37096c9aa6a5a75f00b940b1d03b21e36b0e47e79769f095fe2ab855bd91e3a38756b7d75a9c4549"),
        Some(address!("0xb4950a7fad428434b11c357fa6d4b4bcd3096a5d")),
    ),
];

pub fn test_all() {
    println!("ecrecover test:");
    for (idx, (name, input, expected)) in CASES.iter().enumerate() {
        let result = ec_recover_run(&Bytes::from_static(input), u64::MAX).unwrap();
        if let Some(address) = expected {
            assert_eq!(
                Address::from_slice(&result.bytes[12..]),
                *address,
                "ecrecover#{}",
                name
            );
        } else {
            assert!(result.bytes.is_empty(), "ecrecover#{}", name);
        }
        println!("\tpass: ecrecover#{idx}[{name}]");
    }
    println!("ecrecover test done");
}
