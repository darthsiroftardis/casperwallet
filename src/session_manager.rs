//! # Session Manager

//!	Library to manage all the interactive functions for user operations

use mongodb::{Client, options::ClientOptions,Collection};
use super::account_manager::account_user::User;
use super::storage;
use std::process;
use std::io::{stdin,stdout,Write};

/// All variants of the commands that are currently supported
pub enum Commands {
	Create(String),
	Load(String),
	Delete(String),
	Quit,
	Invalid,
}

/// All variants of commands that can be supported within the session of a specific user
pub enum UserCommands {
	Sign,
	AddTransactionKey(String),
}



/// A simple function to grab input from the user to name either a User or a TransactionKeypair
pub fn grab_name_from_user() -> String {
	stdout().flush().unwrap();
	let mut input = String::new();
	stdin().read_line(&mut input).unwrap();
	let user_input = input.trim();
	user_input.to_string()
}

/// Connect to the database and return a collection object to perform operations with relation to the database
pub fn start_session() -> mongodb::Collection {
	println!("Attempting to connect to database");
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
	println!("Connected to database");

	return collection;
}

/// Start a user session for a specific user, the commands available are determined by the UserCommand enums
pub fn start_user_session(user: &mut User, collection: &Collection) {
	loop {
		println!("Started session for {}", user.name);
		print!("Enter an option\n[1]Sign\n[2]Add transaction key\n[3]List transaction key\n[4]Return\n[Input]:");
		stdout().flush().unwrap();
		let mut input = String::new();
		stdin().read_line(&mut input).unwrap();
		let user_option = input.trim();
		let user_option: u32 = match user_option.parse(){
			Ok(num) => num,
			Err(_e) => 0,
		};
		match user_option {
			1 => println!("Signing"),
			2 => {  
					print!("Enter Key name");
					user.create_new_transaction_key(grab_name_from_user()); 
					storage::update_user(&user, collection);
				 },
			3 => user.list_transaction_keys(),
			4 => break,
			_ => println!("Invalid"),
		}
	}

}

/// Function to execute a specific operation
/// The execution depends on the type command based on the variant of the Commands Enum
pub fn execute_command(collection: &Collection, command: Commands) {
	match command {
		Commands::Create(name) => {
			let mut new_user = User::new(name);
			storage::store_user(&new_user,collection);
			start_user_session(&mut new_user,collection);

		},
		Commands::Load(name) => {
			let mut user = storage::load_user(name,collection);
			start_user_session(&mut user,collection)
		},
		Commands::Delete(name) => {
			storage::delete_user(name, collection);
		},
		Commands::Quit => {
			process::exit(0);
		},
		Commands::Invalid => println!("Invalid Command"),
	}
}