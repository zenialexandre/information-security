use std::io;
use std::io::Write;

use ndarray::Array2;

const FLUSH_FAILED: &str = "Failed to flush.";
const READ_LINE_FAILED: &str = "Failed to read line.";
const PARSE_FAILED: &str = "Failed to parse.";

fn main() {
    initialize_program();
}

fn initialize_program() {
    println!("\nRail Fence Cipher");
    println!("#######################################\n");

    let simple_phrase: String = get_user_simple_phrase();
}

fn get_user_simple_phrase() -> String {
    let mut simple_phrase_input: String = String::new();

    println!("\nPlease inform the phrase to be ciphered: ");
    io::stdout().flush().expect(FLUSH_FAILED);
    io::stdin().read_line(&mut simple_phrase_input).expect(READ_LINE_FAILED);
    return simple_phrase_input;
}
