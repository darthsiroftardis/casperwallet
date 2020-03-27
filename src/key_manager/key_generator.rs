use openssl::ec::*;
use openssl::nid::Nid;
use openssl::error::ErrorStack;
use openssl::pkey::Private;

//use storage;

pub fn generate_account_key() -> Result<EcKey<Private>,ErrorStack> {
	let group = EcGroup::from_curve_name(Nid::SECP256K1).unwrap();
	EcKey::<Private>::generate(&group)
}

pub fn generate_tx_key() -> Result<EcKey<Private>,ErrorStack> {
	let group = EcGroup::from_curve_name(Nid::SECP256K1).unwrap();
	EcKey::<Private>::generate(&group)
}
/*
pub fn get_key(pem: &[u8], passphrase: &[u8],user_name: &String) -> Result<EcKey<Private>,ErrorStack> {
	let name = user_name.clone();
	name.push_str(&String::from("/account.key"));
	let bytes = storage::load_file(&name);
	EcKey::private_key_from_pem_passphrase(pem, passphrase)
}*/