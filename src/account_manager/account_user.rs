//! # User
//! The module to provide the cryptographic and functional utility
/// # User

use ed25519_dalek::Keypair;
use std::collections::HashMap;
use super::key_manager::key_generator;




pub struct User {
	pub name: 					String, //Name 
	pub account_keypair:		Keypair, //Account Keypair
	pub transaction_keypairs: 	HashMap<String,Keypair>, //Hashmap for a list of keypairs indexed by name
}


impl User {
	///Create a new user with a given name. This struct provides the cryptographic and functional utility
	/// # Examples
	/// ```
	/// let user: User = new(String::from("Bob"));
	/// ```
	pub fn new(name: String) -> User {
		let account_keypair: Keypair = key_generator::generate_keypair();
		let transaction_keypairs: HashMap<String,Keypair> = HashMap::new();
		User {
			name,
			account_keypair,
			transaction_keypairs,
		}
	}

	///Add a new transaction key with a given key.

	/// You must create a mutable user to add new transaction keys
	/// # Examples
	/// ```
	/// let mut user: User = User::new(String::from("Ned"));
	/// user.create_new_transaction_key(String::from("NewKey"));
	/// ```
	pub fn create_new_transaction_key(&mut self, key_name: String) {
		let keypair = key_generator::generate_keypair();
		self.transaction_keypairs.insert(key_name, keypair);
	}

	///List current transaction keys 
	/// # Examples
	/// ```
	/// let user: User = User::new(String::from("Homer"));
	///	user.list_transaction_keys()
	/// ```
	pub fn list_transaction_keys(&self) {
		for (name,pair) in &self.transaction_keypairs {
			println!("{:?}", name);
			println!("Private:{:?}", hex::encode(pair.secret.to_bytes()));
			println!("Public:{:?}", hex::encode(pair.public.to_bytes()));
		}
	}	

}



#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	///If test passes then a valid user is created
	fn check_user() {
		let user = User::new(String::from("Bob"));
		assert_eq!(user.name, String::from("Bob"));
	}
}