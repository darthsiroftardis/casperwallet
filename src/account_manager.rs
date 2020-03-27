pub use super::key_manager::key_generator;
pub use super::key_manager;
pub use super::storage;
use account_user::User;


pub mod account_user;

use std::env;
use std::{fs,io};

pub fn list_users(director: &String) -> io::Result<()> {
	env::set_current_dir(director);
	let mut entries = fs::read_dir("")?
		.map(|res| res.map(|e| e.path()))
		.collect::<Result<Vec<_>, io::Error>>()?;
	entries.sort();
	Ok(())
}

pub fn first_setup() {
	let mut user = User::new("ben_dover".to_string(),&"testpassphrase".as_bytes());
	println!("{:?}", user.name);
	user.add_transaction_key(String::from("FirstKey"));
	user.add_transaction_key(String::from("SecondKey"));
	user.list_transaction_keys();
}

