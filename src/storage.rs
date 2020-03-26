use std::fs::File;
use std::io::prelude::*;
use std::path::Path;



pub fn store_to_disk(bytes: &[u8],name: &String) -> std::io::Result<()> {
	let mut name = name.clone();
	let suffix = ".key".to_string();
	name.push_str(&suffix);
	let path = Path::new(&name);
	let _display = path.display();
	let mut file = match File::create(&path) {
		Err(why) => panic!("{:?}", why.to_string()),
		Ok(file) => file,
	};

	file.write_all(bytes)
}