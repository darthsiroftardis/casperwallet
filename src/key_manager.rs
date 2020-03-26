use openssl::error::ErrorStack;
use openssl::symm::Cipher;
use std::io::prelude;
use std::io::BufWriter;
use std::fs::File;

use super::storage;

pub mod key_generator;



pub fn store_key(passphrase: &[u8]) -> std::io::Result<()> {
	let key = match key_generator::generate_account_key() {
		Ok(ec_key) => ec_key,
		Err(why) => panic!("{:?}", why.to_string()),
	};
	let cipher = Cipher::aes_256_gcm();
	let key_bytes = match key.private_key_to_pem_passphrase(cipher, passphrase) {
		Ok(bytes) => bytes,
		Err(why) => panic!("{:?}", why.to_string()),
	};
	storage::store_to_disk(&key_bytes)
}