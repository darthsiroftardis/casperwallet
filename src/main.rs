use casperwallet::account_manager::account_user::User;
use casperwallet::storage;
use casperwallet::account_manager;
use casperwallet::session_manager;
use mongodb::{Client, options::ClientOptions};




fn main() {


	let user = User::new(String::from("Bob"));

	let user_entry = storage::UserEntry::new(user);

	storage::store_user(&user_entry, &collection);
	let u = storage::load_user(String::from("Bob"),&collection);
	println!("{:?}", u.account_keypair);
	storage::delete_user(String::from("Bob"),&collection);


}
