use std::collections::HashMap;
use mongodb::Collection;
use bson;
use bson::{doc,Bson};
use mongodb::options::{FindOneAndDeleteOptions,FindOneOptions};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use ed25519_dalek::SECRET_KEY_LENGTH;
use super::account_manager::account_user::User;


#[derive(Serialize, Deserialize, Debug)]
pub struct UserEntry {
	pub name:								String,
	pub account_secretkey_bytes:			[u8;SECRET_KEY_LENGTH],
	pub transaction_secretkey_bytes: 		HashMap<String,[u8;SECRET_KEY_LENGTH]>,
}


impl UserEntry {
	// add code here
	pub fn new(user: User) -> UserEntry {
	let mut transaction_secretkey_bytes: HashMap<String,[u8;SECRET_KEY_LENGTH]> = HashMap::new();
	for (k,v) in &user.transaction_keypairs {
		println!("{:?}", k.to_string());
		transaction_secretkey_bytes.insert(k.to_string(),v.secret.to_bytes());
	}
	let account_secretkey_bytes = user.account_keypair.secret.to_bytes();
	let name = user.name.clone();
	UserEntry {
		name,
		account_secretkey_bytes,
		transaction_secretkey_bytes,
	}
}
}



pub fn store_user(user: &UserEntry, collection: &Collection) {
	let name = user.name.clone();
	let json_entry = match serde_json::to_string(&user) {
		Ok(jstring) => jstring,
		Err(e) => panic!("{:?}", e),
	};

	let user_entry = doc!{
		"name":name,
		"data":json_entry,
	};

	match collection.insert_one(user_entry, None) {
		Ok(res) => println!("{:?}", res),
		Err(e) => panic!("{:?}", e),
	}
}

pub fn load_user(name: String, collection: &Collection){
	let filter = doc! {"name":name};
	let find_options = FindOneOptions::builder().build();
	let document = match collection.find_one(filter, find_options) {
		Ok(document) => document,
		Err(e) => panic!("{:?}", e),
	};

	if let Some(user) = document.unwrap().get("data").and_then(Bson::as_str) {
		let u = match serde_json::from_str(user) {
			Ok(u) => u,
			Err(e) => panic!("Batman {:?}", e),
		};
	} else {
		panic!("Error in Deserialize");
	}
}





pub fn delete_user(user_name: String, collection: &Collection) {
	let filter = doc!{"name": user_name};
	let find_options = FindOneAndDeleteOptions::builder().build();
	let document = match collection.find_one_and_delete(filter,find_options) {
		Ok(document) => document,
		Err(e) => panic!("{:?}", e),
	}; 
	println!("{:?}|Deleted!", document.unwrap());
}