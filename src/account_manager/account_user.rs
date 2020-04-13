use ed25519_dalek::Keypair;
use std::collections::HashMap;
use super::key_manager::key_generator;


pub struct User {
	pub name: 					String,
	pub account_keypair:		Keypair,
	pub transaction_keypairs: 	HashMap<String,Keypair>,
}


impl User {
	pub fn new(name: String) -> User {
		let account_keypair: Keypair = key_generator::generate_keypair();
		let transaction_keypairs: HashMap<String,Keypair> = HashMap::new();
		User {
			name,
			account_keypair,
			transaction_keypairs,
		}
	}
	pub fn create_new_transaction_key(&mut self, key_name: String) {
		let keypair = key_generator::generate_keypair();
		self.transaction_keypairs.insert(key_name, keypair);
	}

	pub fn list_transaction_keys(&self) {
		for (name,pair) in &self.transaction_keypairs {
			println!("{:?}", name);
			println!("{:?}", hex::encode(pair.secret.to_bytes()));
			println!("{:?}", hex::encode(pair.public.to_bytes()));
		}
	}	

}

/*
impl Serialize for User {
	// add code here
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let mut state = serializer.serialize_struct("User",3)?;
		state.serialize_field("name", &self.name)?;
		state.serialize_field("account_keypair",&self.account_keypair)?;
		state.serialize_field("transaction_keypairs", &self.transaction_keypairs)?;
		state.end()
	}
}
*/







#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn check_user() {
		let user = User::new(String::from("Bob"));
		assert_eq!(user.name, String::from("Bob"));
	}
}