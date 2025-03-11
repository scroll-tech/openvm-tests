#[allow(unused_imports, clippy::single_component_path_imports)]
use {
    openvm::platform as openvm_platform,
    openvm_algebra_guest::IntMod,
    openvm_bigint_guest, // trigger extern u256 (this may be unneeded)
    openvm_ecc_guest::k256::Secp256k1Point,
    //openvm_ecc_guest::p256::P256Point,
    openvm_keccak256_guest, // trigger extern native-keccak256
    openvm_pairing_guest::bn254::Bn254G1Affine,
};

use openvm_ecc_guest::AffinePoint;
use openvm_pairing_guest::{
    bn254::{Bn254, Fp, Fp2},
    pairing::PairingCheck,
};

openvm_algebra_guest::moduli_macros::moduli_init! {
    "0x30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47", // Bn254Fp Coordinate field
    "0x30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001", // Bn254 Scalar
    "0xFFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFE FFFFFC2F", // secp256k1 Coordinate field
    "0xFFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFE BAAEDCE6 AF48A03B BFD25E8C D0364141", // secp256k1 Scalar field
}

openvm_ecc_guest::sw_macros::sw_init! {
    Secp256k1Point,
    //P256Point,
    Bn254G1Affine
}

openvm_algebra_complex_macros::complex_init! {
    Bn254Fp2 { mod_idx = 0 },
}

openvm::entry!(main);

const PAIR_ELEMENT_LEN: usize = 192;

fn main() {
    setup_all_moduli();
    setup_all_curves();
    setup_all_complex_extensions();

    // copied from https://github.com/bluealloy/revm/blob/9e39df5dbc5fdc98779c644629b28b8bee75794a/crates/precompile/src/bn128.rs#L395
    let input = hex::decode(
        "\
            1c76476f4def4bb94541d57ebba1193381ffa7aa76ada664dd31c16024c43f59\
            3034dd2920f673e204fee2811c678745fc819b55d3e9d294e45c9b03a76aef41\
            209dd15ebff5d46c4bd888e51a93cf99a7329636c63514396b4a452003a35bf7\
            04bf11ca01483bfa8b34b43561848d28905960114c8ac04049af4b6315a41678\
            2bb8324af6cfc93537a2ad1a445cfd0ca2a71acd7ac41fadbf933c2a51be344d\
            120a2a4cf30c1bf9845f20c6fe39e07ea2cce61f0c9bb048165fe5e4de877550\
            111e129f1cf1097710d41c4ac70fcdfa5ba2023c6ff1cbeac322de49d1b6df7c\
            2032c61a830e3c17286de9462bf242fca2883585b93870a73853face6a6bf411\
            198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c2\
            1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed\
            090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b\
            12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa",
    )
    .unwrap();

    let elements = input.len() / PAIR_ELEMENT_LEN;

    let mut P = Vec::with_capacity(elements);
    let mut Q = Vec::with_capacity(elements);

    // read points
    for idx in 0..elements {
        // At each idx, there is (G1, G2) which is 6 Fp points
        let read_fq_at = |n: usize| {
            debug_assert!(n < PAIR_ELEMENT_LEN / 32);
            let start = idx * PAIR_ELEMENT_LEN + n * 32;
            // SAFETY: We're reading `6 * 32 == PAIR_ELEMENT_LEN` bytes from `input[idx..]`
            // per iteration. This is guaranteed to be in-bounds.
            let slice = unsafe { input.get_unchecked(start..start + 32) };
            Fp::from_be_bytes(&slice[..32])
        };
        // https://eips.ethereum.org/EIPS/eip-197, Fp2 is encoded as (a, b) where a * i + b
        let g1_x = read_fq_at(0);
        let g1_y = read_fq_at(1);
        let g2_x_c1 = read_fq_at(2);
        let g2_x_c0 = read_fq_at(3);
        let g2_y_c1 = read_fq_at(4);
        let g2_y_c0 = read_fq_at(5);

        let g1 = AffinePoint::new(g1_x, g1_y);
        let g2_x = Fp2::new(g2_x_c0, g2_x_c1);
        let g2_y = Fp2::new(g2_y_c0, g2_y_c1);
        let g2 = AffinePoint::new(g2_x, g2_y);

        P.push(g1);
        Q.push(g2);
    }
    let success = Bn254::pairing_check(&P, &Q).is_ok();
    println!("Pairing check: {}", success);
}
