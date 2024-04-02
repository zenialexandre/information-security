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
    let number_of_rail_fences: usize = get_user_number_of_rail_fences();
    let simple_phrase_rail_fence_matrix: Array2<char> = get_simple_phrase_rail_fance_matrix(
        simple_phrase,
        number_of_rail_fences
    );
}

fn get_user_simple_phrase() -> String {
    let mut simple_phrase_input: String = String::new();

    println!("\nPlease inform the phrase to be ciphered: ");
    io::stdout().flush().expect(FLUSH_FAILED);
    io::stdin().read_line(&mut simple_phrase_input).expect(READ_LINE_FAILED);
    return simple_phrase_input;
}

fn get_user_number_of_rail_fences() -> usize {
    let mut number_of_rail_fences_input: String = String::new();

    println!("\nPlease inform the number of rail fences to be used: ");
    io::stdout().flush().expect(FLUSH_FAILED);
    io::stdin().read_line(&mut number_of_rail_fences_input).expect(READ_LINE_FAILED);
    return number_of_rail_fences_input.trim().parse().expect(PARSE_FAILED);
}

fn get_simple_phrase_rail_fance_matrix(
    simple_phrase: String,
    number_of_rail_fences: usize
) -> Array2<char> {
    let simple_phrase_characters: Vec<char> =
        remove_whitespaces_from_simple_phrase(simple_phrase).chars().collect();
    let mut simple_phrase_rail_fence_matrix: Array2<char> = Array2::from_elem(
        (number_of_rail_fences, simple_phrase_characters.len()),
        ' '
    );

    // TODO: Fill the matrix with the chars.

    println!("\nPhrase matrix: \n{:?}", simple_phrase_rail_fence_matrix);
    return simple_phrase_rail_fence_matrix;
}

fn remove_whitespaces_from_simple_phrase(simple_phrase: String) -> String {
    return simple_phrase.split_whitespace().collect();
}
