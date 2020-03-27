use casperwallet::account_manager::account_user::User;
use casperwallet::account_manager;
use dirs;
use std::env;

fn main() {
/*	let mut directory = match dirs::home_dir() {
		Some(path) => String::from(path.to_str().unwrap()),
		None => panic!("Error"),
	};
	directory.push_str(&String::from("/casperkeys"));
	
	match account_manager::list_users(&directory) {
		Ok(_) => println!("Keys found"),
		Err(_) => println!("No users found"),
	};
*/

	let mut user = User::new("ben_dover_1".to_string(),&"testpassphrase".as_bytes());
	println!("{:?}", user.name);
	user.add_transaction_key(String::from("FirstKey"));
	user.add_transaction_key(String::from("SecondKey"));
	//user.list_transaction_keys();

	let test = user.load_transaction_key();


	println!("{:?}", test);
	

}
