use casperwallet::key_manager::key_generator;
use casperwallet::key_manager;
use casperwallet::storage;

use hex::encode;


fn main() {
    let passphrase = "testpasspharse".as_bytes();
    match key_manager::store_key(&passphrase) {
    	Ok(_) => {
    		println!("Key created and stored");
    	},
    	Err(err) => {
    		println!("{:?}", err);
    	}
    }
}
