use snarkvm_console_algorithms::ToBits;
use snarkvm_console_network::Testnet3;
use snarkvm_console_program::{Literal, LiteralType, Network, Plaintext};

use std::str::FromStr;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = "hash")]
pub fn hash(algorithm: &str, val: &str, output: &str) -> String {
    let input = Plaintext::<Testnet3>::from_str(val).unwrap();
    let destination = LiteralType::from_str(output).unwrap();

    let output = match algorithm {
        "bhp256" => Testnet3::hash_to_group_bhp256(&input.to_bits_le()).unwrap(),
        "bhp512" => Testnet3::hash_to_group_bhp512(&input.to_bits_le()).unwrap(),
        "bhp768" => Testnet3::hash_to_group_bhp768(&input.to_bits_le()).unwrap(),
        "bhp1024" => Testnet3::hash_to_group_bhp1024(&input.to_bits_le()).unwrap(),
        "keccak256" => Testnet3::hash_to_group_bhp256(&Testnet3::hash_keccak256(&input.to_bits_le()).unwrap()).unwrap(),
        "keccak384" => Testnet3::hash_to_group_bhp512(&Testnet3::hash_keccak384(&input.to_bits_le()).unwrap()).unwrap(),
        "keccak512" => Testnet3::hash_to_group_bhp512(&Testnet3::hash_keccak512(&input.to_bits_le()).unwrap()).unwrap(),
        "ped64" => Testnet3::hash_to_group_ped64(&input.to_bits_le()).unwrap(),
        "ped128" => Testnet3::hash_to_group_ped128(&input.to_bits_le()).unwrap(),
        "sha3_256" => Testnet3::hash_to_group_bhp256(&Testnet3::hash_sha3_256(&input.to_bits_le()).unwrap()).unwrap(),
        "sha3_384" => Testnet3::hash_to_group_bhp512(&Testnet3::hash_sha3_384(&input.to_bits_le()).unwrap()).unwrap(),
        "sha3_512" => Testnet3::hash_to_group_bhp512(&Testnet3::hash_sha3_512(&input.to_bits_le()).unwrap()).unwrap(),
        _ => panic!("algorithm not supported")
    };
    let literal_output = Literal::Group(output);
    let casted = literal_output.cast_lossy(destination).unwrap();
    casted.to_string()
}

#[test]
fn test_direct() {
    let algorithm = "bhp256";
    let val = "{arr: [1u8, 1u8], size: 2u8}";
    let destination_type = "address";

    let g = hash(algorithm, val, destination_type);
    // let ans = hash_group_to_type(g, destination_type);

    println!("{:?}", g);
}
