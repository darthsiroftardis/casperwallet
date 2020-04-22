//! # Key Generator
//!  Module for cryptographic key generation


use rand::rngs::OsRng;
use ed25519_dalek::{Keypair,SecretKey,PublicKey};

///Generates the Keypair struct using the ED25519 Curve
/// # Examples
/// ```
///	let keypair = generate_keypair();
/// ```
pub fn generate_keypair() -> ed25519_dalek::Keypair {
	let mut csprng = OsRng{};
	let keypair: Keypair = Keypair::generate(&mut csprng);
	keypair
}


///The Keypair is stored as two seperate public and private keystrings.
///Inorder to maintain the functionaility of the underlying library, the keypair must be recovered when loaded from the database
/// # Examples
/// ```
/// let bytes = [195,172,219,111,57,44,18,178,203,227,169,238,206,170,212,61,183,60,128,214,132,158,229,243,200,89,232,38,243,15,243,70];
/// let keypair: Keypair = recover_keypair(&bytes);
/// ```
pub fn recover_keypair(secret_key_bytes: &[u8]) -> Keypair {
	let secret = SecretKey::from_bytes(secret_key_bytes).unwrap();
	let public = PublicKey::from(&secret);
	Keypair {
		secret,
		public,
	}
}




#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	///Can the keypair checking signing, if this passes we have a valid keypair
	fn check_key_signing() {
		let key: Keypair = generate_keypair();
		let test: &[u8] = b"Test";
		let sign: Signature = key.sign(test);
		assert!(key.verify(test,&sign).is_ok());
	}

	#[test]
	///Is the recovered keypair, a valid keypair, if the test passes, then we have recovered a valid keypair
	fn check_key_recovery() {
		let key: Keypair = generate_keypair();
		let secret_bytes = key.secret.to_bytes();
		let public_bytes = key.public.to_bytes();

		let message: &[u8] = b"Test";

		let signature = key.sign(message);

		let recovered_secret = SecretKey::from_bytes(&secret_bytes).unwrap();
		let recovered_public = PublicKey::from(&recovered_secret);

		let recovered_pair = Keypair {
			secret: recovered_secret,
			public: recovered_public,
		};

		assert!(recovered_pair.verify(message, &signature).is_ok());


	}


}


