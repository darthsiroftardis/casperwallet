use casperwallet::account_manager::account_user::User;
use casperwallet::account_manager;
use casperwallet::session_manager;


use std::env;
use std::path::PathBuf;


fn main() {
	let mut user = User::new(String::from("alice"));
	user.add_transaction_key(String::from("FirstKey"));
	user.add_transaction_key(String::from("SecondKey"));
	let path: PathBuf = std::env::current_dir().unwrap();
	let key = account_manager::load_user_account_key(&user.name,&path);
	println!("{:?}", key);
}
