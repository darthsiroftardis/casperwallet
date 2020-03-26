use openssl::error::ErrorStack;


pub mod key_generator;


pub fn store_key() -> Result<Vec<u8>, ErrorStack> {
	let key = key_generator::generate_account_key();
	match key {
		Ok(ec_key) => {
			ec_key.private_key_to_pem()
		},
		Err(err) => {
			Err(err)
		},
	}
}