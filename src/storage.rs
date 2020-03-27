use std::fs::File;
use std::fs::OpenOptions;
use std::fs;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::env;


pub fn load_file(key_name: &PathBuf) -> Vec<u8> {
	match fs::read(key_name) {
		Ok(bytes) => bytes,
		Err(why) => panic!("{:?}", why.to_string()),
	}
}

pub fn store_to_file(bytes: &[u8],name: String) -> std::io::Result<()> {
	let mut file = OpenOptions::new().create(true).write(true).open(name).expect("file already exists");
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