use crate::constants::path::{
    PRIVATE_KEY_FILE_PATH,
    PUBLIC_KEY_FILE_PATH
};

use rsa::{
    traits::PublicKeyParts,
    RsaPrivateKey,
    RsaPublicKey
};

use std::{
    fs::File,
    io::{
        self,
        prelude::*
    }
};

use rand::rngs::ThreadRng;

pub(crate) fn start_rsa_keys() -> io::Result<()> {
    let (private_key, public_key) = get_rsa_keys();
    print_keys_information(private_key.clone(), public_key.clone());
    generate_keys_file(private_key, public_key)?;

    return Ok(());
}

fn get_rsa_keys() -> (RsaPrivateKey, RsaPublicKey) {
    let mut thread_rng: ThreadRng = rand::thread_rng();
    let bits: usize = 2048;
    let private_key: RsaPrivateKey = RsaPrivateKey::new(&mut thread_rng, bits).expect("Failed");
    let public_key: RsaPublicKey = RsaPublicKey::from(&private_key);

    return (private_key, public_key);
}

fn print_keys_information(
    private_key: RsaPrivateKey,
    public_key: RsaPublicKey
) {
    println!("Information about the keys:\n");
    println!("Private key: ");
    println!("Modulus: {}", private_key.n());
    println!("Exponent: {}", private_key.e());
    println!("\nPublic key: ");
    println!("Modulus: {}", public_key.n());
    println!("Exponent: {}", public_key.e());
}

fn generate_keys_file(
    private_key: RsaPrivateKey,
    public_key: RsaPublicKey
) -> io::Result<()> {
    let mut private_key_file: File = File::create(PRIVATE_KEY_FILE_PATH)?;
    let mut public_key_file: File = File::create(PUBLIC_KEY_FILE_PATH)?;

    writeln!(private_key_file, "{} {}", private_key.n(), private_key.e())?;
    writeln!(public_key_file, "{} {}", public_key.n(), public_key.e())?;

    return Ok(());
}
