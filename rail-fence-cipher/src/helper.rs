use std::io;
use std::io::Write;
use ndarray::Array2;

use crate::constants::general_constants::{
    FLUSH_FAILED,
    READ_LINE_FAILED,
    PARSE_FAILED,
    OPTION_EXTRACTION_FAILED
};

pub fn get_user_simple_phrase(user_objective: &str) -> String {
    let mut simple_phrase_input: String = String::new();

    println!("\nPlease inform the phrase to be {}: ", user_objective);
    io::stdout().flush().expect(FLUSH_FAILED);
    io::stdin().read_line(&mut simple_phrase_input).expect(READ_LINE_FAILED);
    return simple_phrase_input;
}

pub fn get_user_number_of_rail_fences() -> usize {
    let mut number_of_rail_fences_input: String = String::new();

    println!("\nPlease inform the number of rail fences to be used: ");
    io::stdout().flush().expect(FLUSH_FAILED);
    io::stdin().read_line(&mut number_of_rail_fences_input).expect(READ_LINE_FAILED);
    return number_of_rail_fences_input.trim().parse().expect(PARSE_FAILED);
}

pub fn remove_whitespaces_from_simple_phrase(simple_phrase: String) -> String {
    return simple_phrase.split_whitespace().collect();
}

pub fn get_rail_fences_populated(
    simple_phrase_characters: Vec<char>,
    number_of_rail_fences: usize,
    mut simple_phrase_matrix: Array2<char>,
    is_deciphering: bool,
    print_matrix: bool
) -> (Array2<char>, String) {
    let mut is_direction_down: bool = false;
    let mut rail_fence: usize = 0;
    let mut extracted_simple_phrase: String = String::new();

    for index in 0..simple_phrase_characters.len() {
        if rail_fence == 0 || rail_fence == number_of_rail_fences - 1 {
            is_direction_down = !is_direction_down
        }
        extracted_simple_phrase.push_str(simple_phrase_matrix[[rail_fence, index]].to_string().as_str());

        if !is_deciphering {
            simple_phrase_matrix[[rail_fence, index]] =
                *simple_phrase_characters.get(index).ok_or(OPTION_EXTRACTION_FAILED).unwrap();
        } else {
            simple_phrase_matrix[[rail_fence, index]] = '@';
        }

        if is_direction_down { rail_fence += 1; } else { rail_fence -= 1; }
    }

    if print_matrix {
        println!("\nPhrase matrix: ");
        println!("\n{:?}", simple_phrase_matrix);
    }
    return (simple_phrase_matrix, extracted_simple_phrase);
}
