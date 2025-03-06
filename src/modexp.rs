use crate::should_eq;
use hex_literal::hex;
use revm_precompile::modexp::run_inner;

const CASES: &[(&str, &[u8], &[u8])] = &[
    (
        "go-ethereum/core/vm/testdata/precompiles/modexp.json#eip_example1",
        &hex!("00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000002003fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2efffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f"),
        &hex!("0000000000000000000000000000000000000000000000000000000000000001"),
    ),
    (
        "go-ethereum/core/vm/testdata/precompiles/modexp.json#eip_example2",
        &hex!("000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000020fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2efffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f"),
        &hex!("0000000000000000000000000000000000000000000000000000000000000000"),
    ),
    (
        "go-ethereum/core/vm/testdata/precompiles/modexp.json#eip_example3",
        &hex!("0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000003fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2efffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f"),
        &hex!("0000000000000000000000000000000000000000000000000000000000000001"),
    ),
    (
        "go-ethereum/core/vm/testdata/precompiles/modexp.json#eip_example4",
        &hex!("0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000000fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2efffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f"),
        &hex!("0000000000000000000000000000000000000000000000000000000000000000"),
    ),
];

pub fn test_all() {
    println!("modexp test:");
    for (idx, (name, input, expected)) in CASES.iter().enumerate() {
        let outcome = run_inner(input, u64::MAX, 0, |_, _, _, _| 0).unwrap();
        should_eq!(*outcome.bytes, *expected, "modexp#{idx}[{name}]");
    }
    println!("modexp test done");
}
