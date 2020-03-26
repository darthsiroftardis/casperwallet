use casperwallet::account_manager::account_user::User;

fn main() {
	let user = User::new("ben_dover".to_string(),&"testpassphrase".as_bytes());
	println!("{:?}", user.name);
}
