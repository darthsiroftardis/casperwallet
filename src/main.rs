use casperwallet::session_manager;
use casperwallet::session_manager::Commands;
use std::io::{stdin,stdout,Write};



fn main() {
	let collection = session_manager::start_session();
	loop {
		print!("Enter an option\n[1]Create\n[2]Load\n[3]Delete\n[4]Quit\n[Input]:");
		stdout().flush().unwrap();
		let mut input = String::new();
		stdin().read_line(&mut input).unwrap();
		let user_option = input.trim();
		let user_option: u32 = match user_option.parse(){
			Ok(num) => num,
			Err(_e) => 0,
		};
		let command = match user_option {
			1 => Commands::Create(String::from("Bob")),
			2 => Commands::Load(String::from("Bob")),
			3 => Commands::Delete(String::from("Bob")),
			4 => Commands::Quit,
			_ => Commands::Invalid,
		};
		session_manager::execute_command(&collection,command)
	}




}
