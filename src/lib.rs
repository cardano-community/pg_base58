use pgrx::*;
use bs58;

pg_module_magic!();

#[pg_extern]
fn base58_encode(input: &[u8]) -> String {
    bs58::encode(input).into_string()
}

#[pg_extern]
fn base58_decode(input: &str) -> Vec<u8> {
    bs58::decode(input).into_vec().expect("Failed to decode Base58 string")
}