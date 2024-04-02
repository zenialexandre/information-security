use std::io;
use std::io::Write;

use ndarray::{
    Array2,
    Axis
};

const FLUSH_FAILED: &str = "Failed to flush.";
const READ_LINE_FAILED: &str = "Failed to read line.";
const PARSE_FAILED: &str = "Failed to parse.";

fn main() {
    initialize_program();
}

fn initialize_program() {
    println!("\nGeometric Columnar Transposition Cipher");
    println!("#######################################\n");

    let simple_phrase: String = get_user_simple_phrase();
    let number_of_columns: usize = get_user_number_of_columns();
    let simple_phrase_matrix: Array2<char> = get_simple_phrase_matrix(
        simple_phrase,
        number_of_columns
    );

    cipher_simple_phrase(simple_phrase_matrix.clone());
    decipher_simple_phrase(simple_phrase_matrix);
}

fn get_user_simple_phrase() -> String {
    let mut simple_phrase_input: String = String::new();

    println!("\nPlease inform the phrase to be ciphered: ");
    io::stdout().flush().expect(FLUSH_FAILED);
    io::stdin().read_line(&mut simple_phrase_input).expect(READ_LINE_FAILED);
    return simple_phrase_input;
}

fn get_user_number_of_columns() -> usize {
    let mut number_of_columns_input: String = String::new();

    println!("\nPlease inform the number of columns to the matrix: ");
    io::stdout().flush().expect(FLUSH_FAILED);
    io::stdin().read_line(&mut number_of_columns_input).expect(READ_LINE_FAILED);
    return number_of_columns_input.trim().parse().expect(PARSE_FAILED);
}

fn get_simple_phrase_matrix(
    simple_phrase: String,
    number_of_columns: usize
) -> Array2<char> {
    let simple_phrase_characters: Vec<char> =
        remove_whitespaces_from_simple_phrase(simple_phrase).chars().collect();
    let number_of_rows: usize = get_number_of_rows(
        simple_phrase_characters.clone(),
        number_of_columns
    );
    let mut simple_phrase_matrix: Array2<char> = Array2::from_elem(
        (number_of_rows, number_of_columns),
        ' '
    );

    simple_phrase_matrix = set_phrase_characters_on_matrix(
        simple_phrase_characters,
        simple_phrase_matrix.clone()
    );
    simple_phrase_matrix = fill_empty_cells_on_matrix(simple_phrase_matrix);

    println!("\nPhrase matrix: \n{:?}", simple_phrase_matrix);
    return simple_phrase_matrix;
}

fn remove_whitespaces_from_simple_phrase(simple_phrase: String) -> String {
    return simple_phrase.split_whitespace().collect();
}

fn get_number_of_rows(
    simple_phrase_characters: Vec<char>,
    number_of_columns: usize
) -> usize {
    return (simple_phrase_characters.len() + (number_of_columns - 1)) / number_of_columns;
}

fn set_phrase_characters_on_matrix(
    simple_phrase_characters: Vec<char>,
    mut simple_phrase_matrix: Array2<char>
) -> Array2<char> {
    for (iteration_index, character) in simple_phrase_characters.iter().enumerate() {
        let simple_phrase_matrix_row: usize = iteration_index / simple_phrase_matrix.len_of(Axis(1));
        let simple_phrase_matrix_column: usize = iteration_index % simple_phrase_matrix.len_of(Axis(1));
        simple_phrase_matrix[[simple_phrase_matrix_row, simple_phrase_matrix_column]] = *character;
    }
    return simple_phrase_matrix;
}

fn fill_empty_cells_on_matrix(mut simple_phrase_matrix: Array2<char>) -> Array2<char> {
    for cell_value in simple_phrase_matrix.iter_mut() {
        if *cell_value == ' ' {
            *cell_value = 'X';
        }
    }
    return simple_phrase_matrix;
}

fn cipher_simple_phrase(simple_phrase_matrix: Array2<char>) {
    let mut ciphered_simple_phrase_output: String = String::new();
    
    for simple_phrase_matrix_column in simple_phrase_matrix.axis_iter(Axis(1)) {
        let column_cell_values: String = simple_phrase_matrix_column.iter().collect();
        ciphered_simple_phrase_output.push_str(&column_cell_values);
    }
    println!("\nCiphered phrase: {}", ciphered_simple_phrase_output);
}

fn decipher_simple_phrase(simple_phrase_matrix: Array2<char>) {
    let mut deciphered_simple_phrase_output: String = String::new();

    for cell_value in simple_phrase_matrix.iter() {
        deciphered_simple_phrase_output.push_str(cell_value.to_string().as_str());
    }
    println!("\nDeciphered phrase: {}", deciphered_simple_phrase_output);
}
