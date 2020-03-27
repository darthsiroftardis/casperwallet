use openssl::symm::Cipher;
use openssl::ec::EcKey;
use openssl::pkey::Private;
use openssl::error::ErrorStack;

use super::storage;

pub mod key_generator;

pub fn store_account_key(passphrase: &[u8], key: &EcKey<Private>, key_name: &String) -> std::io::Result<()> {
	let cipher = Cipher::aes_256_gcm();
	let key_bytes = match key.private_key_to_pem_passphrase(cipher, passphrase) {
		Ok(bytes) => bytes,
		Err(why) => panic!("{:?}", why.to_string()),
	};
	storage::store_to_disk(&key_bytes,key_name)
}

pub fn store_transaction_key(key: &EcKey<Private>, key_name: &String, account_name: &String) -> std::io::Result<()> {
	let key_bytes = match key.private_key_to_pem() {
		Ok(bytes) => bytes,
		Err(why) => panic!("{:?}", why.to_string()),
	};
	storage::store_transaction_key(&key_bytes, key_name, account_name)
}

pub fn load_transaction_key(key_name: &String) -> Result<EcKey<Private>,ErrorStack> {
	let pem_bytes = storage::load_file(key_name);
	EcKey::private_key_from_pem(&pem_bytes)
}

