
use openssl::ec::*;
use openssl::nid::Nid;
use openssl::error::ErrorStack;
use openssl::pkey::Private;

use super::key_manager::key_generator;

pub struct User {
	pub name: String,
	account_key: EcKey<Private>,
	transaction_keys: Vec<EcKey<Private>>,
}

impl User {
	// add code here
	pub fn new(name: String) -> User {
		let account_key = match key_generator::generate_account_key() {
			Ok(ec_key) => ec_key,
			Err(why) => panic!("{:?}", why.to_string()),
		};
		let transaction_keys = vec![];
		User {
			name,
			account_key,
			transaction_keys,
		}
	}

	


}