use openssl::symm::Cipher;
use openssl::ec::EcKey;
use openssl::pkey::Private;

use super::storage;

pub mod key_generator;

pub fn store_key(passphrase: &[u8], key: &EcKey<Private>, key_name: &String) -> std::io::Result<()> {
	let cipher = Cipher::aes_256_gcm();
	let key_bytes = match key.private_key_to_pem_passphrase(cipher, passphrase) {
		Ok(bytes) => bytes,
		Err(why) => panic!("{:?}", why.to_string()),
	};
	storage::store_to_disk(&key_bytes,key_name)
}



