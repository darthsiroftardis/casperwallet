use openssl::ec::*;
use openssl::nid::Nid;
use openssl::error::ErrorStack;
use openssl::pkey::Private;


pub fn generate_account_key() -> Result<EcKey<Private>,ErrorStack> {
	let group = EcGroup::from_curve_name(Nid::SECP256K1).unwrap();
	EcKey::<Private>::generate(&group)
}

pub fn get_key(pem: &[u8], passphrase: &[u8]) -> Result<EcKey<Private>,ErrorStack> {
	EcKey::private_key_from_pem_passphrase(pem, passphrase)
}