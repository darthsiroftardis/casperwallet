use rand::rngs::OsRng;
use ed25519_dalek::{Keypair,SecretKey,PublicKey,Signature};



//Key Generation function to create a Ed25519 Secret Key
pub fn generate_keypair() -> ed25519_dalek::Keypair {
	let mut csprng = OsRng{};
	let keypair: Keypair = Keypair::generate(&mut csprng);
	keypair
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn check_key_signing() {
		let key: Keypair = generate_keypair();
		let test: &[u8] = b"Test";
		let sign: Signature = key.sign(test);
		assert!(key.verify(test,&sign).is_ok());
	}

	#[test]
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


