mod constants;
mod helper;

use ndarray::Array2;
use constants::general_constants::OPTION_EXTRACTION_FAILED;

fn main() {
    initialize_program();
}

fn initialize_program() {
    println!("\nRail Fence Cipher");
    println!("#######################################\n");
    execute_cipher_process();
    execute_decipher_process();
}

fn execute_cipher_process() {
    println!("Initializing the ciphering...");
    let simple_phrase: String = helper::get_user_simple_phrase("ciphered");
    let number_of_rail_fences: usize = helper::get_user_number_of_rail_fences();
    let simple_phrase_matrix: Array2<char> = get_simple_phrase_matrix(
        simple_phrase,
        number_of_rail_fences,
        false
    );
    println!("\nCiphered phrase: {}\n", get_ciphered_simple_phrase(simple_phrase_matrix));
}

fn execute_decipher_process() {
    println!("Initializing the deciphering...");
    let simple_phrase: String = helper::get_user_simple_phrase("deciphered");
    let number_of_rail_fences: usize = helper::get_user_number_of_rail_fences();
    let simple_phrase_matrix: Array2<char> = get_simple_phrase_matrix(
        simple_phrase.clone(),
        number_of_rail_fences,
        true
    );
    println!("\nDeciphered phrase: {}\n", get_deciphered_simple_phrase(
        simple_phrase,
        number_of_rail_fences,
        simple_phrase_matrix,
        true
    ));
}

fn get_simple_phrase_matrix(
    simple_phrase: String,
    number_of_rail_fences: usize,
    is_deciphering: bool
) -> Array2<char> {
    let simple_phrase_characters: Vec<char> =
        helper::remove_whitespaces_from_simple_phrase(simple_phrase).chars().collect();
    let mut simple_phrase_matrix: Array2<char> = Array2::from_elem(
        (number_of_rail_fences, simple_phrase_characters.len()),
        '-'
    );

    simple_phrase_matrix = helper::get_rail_fences_populated(
        simple_phrase_characters,
        number_of_rail_fences,
        simple_phrase_matrix,
        is_deciphering,
        true
    ).0;
    return simple_phrase_matrix;
}

fn get_ciphered_simple_phrase(simple_phrase_matrix: Array2<char>) -> String {
    let mut ciphered_simple_phrase_output: String = String::new();

    for cell_value in &simple_phrase_matrix {
        if *cell_value != '-' {
            ciphered_simple_phrase_output.push_str(cell_value.to_string().as_str());
        }
    }
    return ciphered_simple_phrase_output;
}

fn get_deciphered_simple_phrase(
    simple_phrase: String,
    number_of_rail_fences: usize,
    mut simple_phrase_matrix: Array2<char>,
    is_deciphering: bool
) -> String {
    let simple_phrase_characters: Vec<char> =
        helper::remove_whitespaces_from_simple_phrase(simple_phrase).chars().collect();

    simple_phrase_matrix = get_simple_phrase_matrix_without_arbitrary_characters(
        simple_phrase_characters.clone(),
        simple_phrase_matrix
    );

    return helper::get_rail_fences_populated(
        simple_phrase_characters,
        number_of_rail_fences,
        simple_phrase_matrix,
        is_deciphering,
        false
    ).1;
}

fn get_simple_phrase_matrix_without_arbitrary_characters(
    simple_phrase_characters: Vec<char>,
    mut simple_phrase_matrix: Array2<char>
) -> Array2<char> {
    let mut temporary_simple_phrase_characters: Vec<char> = simple_phrase_characters;

    for cell_value in &mut simple_phrase_matrix {
        if *cell_value == '@' {
            *cell_value = *temporary_simple_phrase_characters.first().ok_or(OPTION_EXTRACTION_FAILED).unwrap();
            Some(temporary_simple_phrase_characters.remove(0));
        }
    }
    return simple_phrase_matrix;
}
