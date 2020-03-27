use openssl::ec::*;
use openssl::pkey::Private;

use super::key_manager::key_generator;
use super::key_manager;

pub struct User {
	pub name: 				String,
	pub	account_key: 		EcKey<Private>,
		transaction_keys: 	Vec<TxKey>,
}

pub struct TxKey {
	pub name: 	String,
		key:	EcKey<Private>,
}

impl User {
	// add code here
	pub fn new(name: String) -> User {
		let account_key = match key_generator::generate_account_key() {
			Ok(ec_key) => ec_key,
			Err(why) => panic!("{:?}", why.to_string()),
		};
		let transaction_keys = vec![];
		match key_manager::store_account_key(&account_key,&name) {
			Ok(_) => println!("User account created and stored"),
			Err(why) => println!("User already exists"),
		};
		User {
			name,
			account_key,
			transaction_keys,
		}
	}

	pub fn add_transaction_key(&mut self, name: String) {
		let key = match key_generator::generate_tx_key() {
			Ok(ec_key) => ec_key,
			Err(why) => panic!("{:?}", why.to_string()),
		};
		match key_manager::store_transaction_key(&key, &name, &self.name) {
			Ok(_) => println!("Transction key stored"),
			Err(why) => panic!("{:?}", why.to_string()),
 		}
		self.transaction_keys.push(
			TxKey {
				name,
				key,
			});
	}

	pub fn list_transaction_keys(self) {
		for tx_key in self.transaction_keys {
			let bytes = tx_key.key.private_key_to_pem().unwrap();
			println!("{}||{}", tx_key.name, hex::encode(bytes));
		}
	}

	pub fn load_transaction_key(self) -> EcKey<Private> {
		let mut key_path = self.name.clone();
		key_path.push_str(&String::from("/FirstKey.key"));
		key_manager::load_transaction_key(&key_path).unwrap()
	}
}