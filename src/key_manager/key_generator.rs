use rand::rngs::OsRng;
use ed25519_dalek::Keypair;



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

}


