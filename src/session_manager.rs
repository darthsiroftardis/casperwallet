use std::io::stdin;
use std::io::Write;
use std::io::stdout;


pub fn wallet_session() {
	let mut input = String::new();
	while input != String::from("QUIT") {
		print!("> ");
		stdout().flush();
		stdin().read_line(&mut input).unwrap();
		let command = input.trim();
		println!("{:?}",command);
	}
}