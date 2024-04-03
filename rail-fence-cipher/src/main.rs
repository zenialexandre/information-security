mod helper;

use ndarray::Array2;

const OPTION_EXTRACTION_ERROR: &str = "Character could not be extracted.";

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
        simple_phrase_matrix
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

    simple_phrase_matrix = set_phrase_characters_on_matrix(
        simple_phrase_characters,
        number_of_rail_fences,
        simple_phrase_matrix,
        is_deciphering
    );
    return simple_phrase_matrix;
}

fn set_phrase_characters_on_matrix(
    simple_phrase_characters: Vec<char>,
    number_of_rail_fences: usize,
    mut simple_phrase_matrix: Array2<char>,
    is_deciphering: bool
) -> Array2<char> {
    let mut is_direction_down: bool = false;
    let mut rail_fence: usize = 0;

    for index in 0..simple_phrase_characters.len() {
        if rail_fence == 0 || rail_fence == number_of_rail_fences - 1 {
            is_direction_down = !is_direction_down
        }

        if !is_deciphering {
            simple_phrase_matrix[[rail_fence, index]] =
                *simple_phrase_characters.get(index).ok_or(OPTION_EXTRACTION_ERROR).unwrap();
        } else {
            simple_phrase_matrix[[rail_fence, index]] = '@';
        }

        if is_direction_down { rail_fence += 1; } else { rail_fence -= 1; }
    }
    println!("\nPhrase matrix: ");
    println!("\n{:?}", simple_phrase_matrix);
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
    mut simple_phrase_matrix: Array2<char>
) -> String {
    let mut deciphered_simple_phrase_output: String = String::new();
    let simple_phrase_characters: Vec<char> =
        helper::remove_whitespaces_from_simple_phrase(simple_phrase).chars().collect();

    simple_phrase_matrix = get_simple_phrase_matrix_without_arbitrary_characters(
        simple_phrase_characters.clone(),
        simple_phrase_matrix
    );

    deciphered_simple_phrase_output = get_extracted_deciphered_simple_phrase_from_matrix(
        deciphered_simple_phrase_output,
        number_of_rail_fences,
        simple_phrase_characters,
        simple_phrase_matrix
    );
    return deciphered_simple_phrase_output;
}

fn get_simple_phrase_matrix_without_arbitrary_characters(
    simple_phrase_characters: Vec<char>,
    mut simple_phrase_matrix: Array2<char>
) -> Array2<char> {
    let mut temporary_simple_phrase_characters: Vec<char> = simple_phrase_characters;

    for cell_value in &mut simple_phrase_matrix {
        if *cell_value == '@' {
            *cell_value = *temporary_simple_phrase_characters.first().ok_or(OPTION_EXTRACTION_ERROR).unwrap();
            Some(temporary_simple_phrase_characters.remove(0));
        }
    }
    return simple_phrase_matrix;
}

fn get_extracted_deciphered_simple_phrase_from_matrix(
    mut deciphered_simple_phrase_output: String,
    number_of_rail_fences: usize,
    simple_phrase_characters: Vec<char>,
    simple_phrase_matrix: Array2<char>
) -> String {
    let mut is_direction_down: bool = false;
    let mut rail_fence: usize = 0;

    for index in 0..simple_phrase_characters.len() {
        if rail_fence == 0 || rail_fence == number_of_rail_fences - 1 {
            is_direction_down = !is_direction_down
        }
        deciphered_simple_phrase_output.push_str(simple_phrase_matrix[[rail_fence, index]].to_string().as_str());

        if is_direction_down { rail_fence += 1; } else { rail_fence -= 1; }
    }
    return deciphered_simple_phrase_output;
}
