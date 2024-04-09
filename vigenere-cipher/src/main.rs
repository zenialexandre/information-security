use std::io;
use std::io::Write;

const FLUSH_FAILED: &str = "Failed to flush.";
const READ_LINE_FAILED: &str = "Failed to read line.";

fn main() {
    initialize_program();
}

fn initialize_program() {
    println!("\nVigenere Cipher");
    println!("#######################################\n");
    execute_cipher_process();
    execute_decipher_process();
}

fn execute_cipher_process() {
    println!("...");
}

fn get_user_simple_phrase(user_objective: &str) -> String {
    let mut user_simple_phrase_input: String = String::new();

    println!("\nPlease inform the phrase to be {}: ", user_objective);
    io::stdout().flush().expect(FLUSH_FAILED);
    io::stdin().read_line(&mut user_simple_phrase_input).expect(READ_LINE_FAILED);
    return user_simple_phrase_input;
}

fn get_user_vigenere_key() -> String {
    let mut user_vigenere_key_input: String = String::new();

    println!("\nPlease inform the key to be used: ");
    io::stdout().flush.expect(FLUSH_FAILED);
    io::stdin().read_line(&mut user_vigenere_key_input).expect(READ_LINE_FAILED);
    return user_vigenere_key_input;
}
