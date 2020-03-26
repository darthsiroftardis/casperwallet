use casperwallet::key_manager::key_generator;
use casperwallet::key_manager;

use hex::encode;


fn main() {
    println!("Hello, world!");
    match key_manager::store_key() {
    	Ok(bytes) => {
    		let hex_string: String = hex::encode(bytes);
    		println!("{:?}", hex_string);
    	},
    	Err(err) => {
    		println!("{:?}", err);
    	},
    }
}
