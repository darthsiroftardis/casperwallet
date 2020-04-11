use casperwallet::account_manager::account_user::User;
use casperwallet::storage;
use casperwallet::account_manager;
use casperwallet::session_manager;
use mongodb::{Client, options::ClientOptions};




fn main() {
	let mut client_options = match ClientOptions::parse("mongodb://localhost:27017") {
		Err(e) => panic!("{:?}", e),
		Ok(co) => co,
	};
	client_options.app_name = Some("test".to_string());
	let client = Client::with_options(client_options).unwrap();
	for db_name in client.list_database_names(None) {
		println!("{:?}", db_name);
	}

	let db = client.database("testdb");
	for collection_name in db.list_collection_names(None) {
		println!("{:?}", collection_name);
	}

	let collection = db.collection("test_keys");

	let user = User::new(String::from("Bob"));

	let user_entry = storage::UserEntry::new(user);

	storage::store_user(&user_entry, &collection);
	storage::delete_user(String::from("Bob"),&collection);


}
