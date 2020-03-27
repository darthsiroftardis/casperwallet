use std::fs::File;
use std::fs;
use std::io::prelude::*;
use std::path::Path;
use std::env;


pub fn load_file(key_name: &String) -> Vec<u8> {
	match env::current_dir() {
		Ok(path) => println!("{:?}", path),
		Err(e) => panic!("{:?}", e),
	};
	println!("{:?}", key_name);
	match fs::read(key_name) {
		Ok(bytes) => bytes,
		Err(why) => panic!("{:?}", why.to_string()),
	}
}

pub fn store_to_file(bytes: &[u8],name: String) -> std::io::Result<()> {
	let path = Path::new(&name);
	let _display = path.display();
	let mut file = match File::create(&path) {
		Err(why) => panic!("{:?}", why.to_string()),
		Ok(file) => file,
	};
	file.write_all(bytes)
}

pub fn store_to_disk(bytes: &[u8], name: &String) -> std::io::Result<()> {
	let mut name = name.clone();
	fs::create_dir(&name)?;
	let suffix = "/account.key".to_string();
	name.push_str(&suffix);
	store_to_file(bytes, name)

}

pub fn store_transaction_key(key_bytes: &[u8], key_name: &String, name: &String) -> std::io::Result<()> {
	let mut name = name.clone();
	name.push_str(&String::from("/"));
	name.push_str(key_name);
	name.push_str(&String::from(".key"));
	store_to_file(key_bytes, name)
}