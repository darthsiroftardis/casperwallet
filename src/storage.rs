use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;



pub fn store_to_disk(bytes: &[u8]) -> std::io::Result<()> {
	let path = Path::new("test.key");
	let display = path.display();
	let mut file = match File::create(&path) {
		Err(why) => panic!("{:?}", why.to_string()),
		Ok(file) => file,
	};

	file.write_all(bytes)
}