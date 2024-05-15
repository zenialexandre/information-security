use crate::constants::path::{
    USER_FILE_PATH_PLACEHOLDER,
    PRIVATE_KEY_FILE_PATH,
    PUBLIC_KEY_FILE_PATH
};

use std::{
    path::Path,
    fs::File,
    io::{
        self,
        prelude::*,
        BufReader,
        Lines,
        Read,
        Result
    }
};

use rand::rngs::ThreadRng;
use rsa::{BigUint, Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use hex::encode;

pub(crate) fn start_crypt(
    input_file_path: String
) -> Result<()> {
    let input_file: File = File::open(input_file_path.clone())?;
    let input_file_data: Lines<BufReader<File>> = io::BufReader::new(input_file).lines();
    let mut input_file_data_str: String = String::new();
    let user_path: &Path = Path::new(&input_file_path);
    let user_file_name: String = user_path.file_name().unwrap().to_str().unwrap().to_string();
    let final_user_path: String = format!("{}{}", USER_FILE_PATH_PLACEHOLDER, user_file_name);

    for row in input_file_data {
        if let Ok(_row) = row {
            println!("\nSimple phrase: {}", _row);
            input_file_data_str = _row;
        }
    }
    let mut user_file: File = File::create(final_user_path)?;
    write!(user_file, "{}", input_file_data_str)?;
    let (ciphered_text_as_str, public_key_exponent_as_bytes) = cipher(input_file_data_str)?;
    decipher(ciphered_text_as_str.as_bytes(), public_key_exponent_as_bytes)?;

    return Ok(());
}

fn cipher(
    input_file_data_str: String
) -> Result<(String, BigUint)> {
    let mut thread_rng: ThreadRng = rand::thread_rng();
    let mut public_key_file: BufReader<File> = BufReader::new(File::open(PUBLIC_KEY_FILE_PATH)?);
    let mut public_key_file_data: String = String::new();

    public_key_file.read_to_string(&mut public_key_file_data)?;

    let splitted_public_key_file_data: Vec<&str> = public_key_file_data.split_ascii_whitespace().collect();
    let public_key_modulus: String = splitted_public_key_file_data.clone().get(0).unwrap().to_string();
    let public_key_exponent: String = splitted_public_key_file_data.clone().get(1).unwrap().to_string();

    let public_key_modulus_as_bytes: BigUint = BigUint::parse_bytes(public_key_modulus.as_bytes(), 10).unwrap();
    let public_key_exponent_as_bytes: BigUint = BigUint::parse_bytes(public_key_exponent.as_bytes(), 10).unwrap();
    let public_key: RsaPublicKey = RsaPublicKey::new(public_key_modulus_as_bytes, public_key_exponent_as_bytes.clone()).unwrap();

    let ciphered_text: Vec<u8> = public_key.encrypt(&mut thread_rng, Pkcs1v15Encrypt, &input_file_data_str.as_bytes()[..]).expect("Failed to cipher");
    let ciphered_text_as_str: String = encode(&ciphered_text);

    println!("{}", ciphered_text_as_str);
    return Ok((ciphered_text_as_str, public_key_exponent_as_bytes));
}

fn decipher(
    ciphered_text: &[u8],
    public_key_exponent_as_bytes: BigUint
) -> Result<()> {
    /*
    TODO:
    let mut private_key_file: BufReader<File> = BufReader::new(File::open(PRIVATE_KEY_FILE_PATH)?);
    let mut private_key_file_data: String = String::new();

    private_key_file.read_to_string(&mut private_key_file_data)?;

    let splitted_private_key_file_data: Vec<&str> = private_key_file_data.split_ascii_whitespace().collect();
    let private_key_modulus: String = splitted_private_key_file_data.clone().get(0).unwrap().to_string();
    let private_key_exponent: String = splitted_private_key_file_data.clone().get(1).unwrap().to_string();

    let private_key_modulus_as_bytes: BigUint = BigUint::parse_bytes(private_key_modulus.as_bytes(), 10).unwrap();
    let private_key_exponent_as_bytes: BigUint = BigUint::parse_bytes(private_key_exponent.as_bytes(), 10).unwrap();
    let public_key: RsaPublicKey = RsaPublicKey::new(public_key_modulus_as_bytes, public_key_exponent_as_bytes.clone()).unwrap();

    let deciphered_text: Vec<u8> = private_key.decrypt(Pkcs1v15Encrypt, ciphered_text).unwrap();
    let deciphered_text_as_str: String = encode(&deciphered_text);

    println!("{}", deciphered_text_as_str);
    return Ok(());
    */
    Ok(())
}
