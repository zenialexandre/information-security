use rsa::{
    traits::PublicKeyParts,
    RsaPrivateKey,
    RsaPublicKey
};

use std::{
    fs::File,
    io::prelude::*
};

use rand::rngs::ThreadRng;

pub(crate) fn start_crypt(input_file_path: String) {
    let (private_key, public_key) = get_rsa_keys();
    print_keys_information(private_key.clone(), public_key.clone());
    generate_keys_file(private_key, public_key);
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
) {
    let mut keys_file: File = File::create("./src/content/rsa_keys.txt").expect("Failed");

    writeln!(keys_file, "Private key: ").expect("Failed");
    writeln!(keys_file, "Modulus: {}", private_key.n()).expect("Failed");
    writeln!(keys_file, "\nPublic key: ").expect("Failed");
    writeln!(keys_file, "Modulus: {}", public_key.n()).expect("Failed");
}
