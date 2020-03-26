use openssl::ec::*;
use openssl::pkey::Private;

use super::key_manager::key_generator;
use super::key_manager;

pub struct User {
	pub name: String,
	account_key: EcKey<Private>,
	transaction_keys: Vec<EcKey<Private>>,
}

impl User {
	// add code here
	pub fn new(name: String, passphrase: &[u8]) -> User {
		let account_key = match key_generator::generate_account_key() {
			Ok(ec_key) => ec_key,
			Err(why) => panic!("{:?}", why.to_string()),
		};
		let transaction_keys = vec![];
		match key_manager::store_key(passphrase,&account_key,&name) {
			Ok(_) => println!("User account created and stored"),
			Err(why) => panic!("{:?}", why.to_string()),
		};
		User {
			name,
			account_key,
			transaction_keys,
		}
	}
}