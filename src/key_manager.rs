use openssl::ec::EcKey;
use openssl::pkey::Private;
use openssl::error::ErrorStack;
use std::path::PathBuf;

pub use super::storage;

pub mod key_generator;


pub fn store_account_key(key: &EcKey<Private>, key_name: &String) -> std::io::Result<()> {
	let key_bytes = match key.private_key_to_pem() {
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
	let mut path = PathBuf::new();
	path.push(key_name);
	let pem_bytes = storage::load_file(&path);
	EcKey::private_key_from_pem(&pem_bytes)
}

pub fn load_account_key(name: &PathBuf) -> Result<EcKey<Private>,ErrorStack> {
	let mut name = name.clone();
	let bytes = storage::load_file(&name);
	println!("{:?}", bytes);
	EcKey::private_key_from_pem(&bytes)
}