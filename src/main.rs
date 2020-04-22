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
			1 => {
				print!("Enter Name for User:");
				let name = session_manager::grab_name_from_user();
				Commands::Create(name)	
			},
			2 => {
				print!("Enter Name for User:");
				let name = session_manager::grab_name_from_user();
				println!("{:?}", name);
				Commands::Load(name)
			},
			3 => {
				print!("Enter Name for User:");
				let name = session_manager::grab_name_from_user();
				Commands::Delete(name)
			},
			4 => Commands::Quit,
			_ => Commands::Invalid,
		};
		session_manager::execute_command(&collection,command)
	}




}
