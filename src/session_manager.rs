use mongodb::{Client, options::ClientOptions};



enum Commands {
	Create(String),
	Load(String),
	Delete(String)
	Quit
}

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

pub fn run_session(command: Commands) {
	loop {
		if command == Commands::Quit {break;}
		execute_command(collection,command);
	}
}




pub fn execute_command(collection: &Collection, command: Commands) {

}