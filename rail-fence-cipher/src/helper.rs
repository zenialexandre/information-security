use std::io;
use std::io::Write;

const FLUSH_FAILED: &str = "Failed to flush.";
const READ_LINE_FAILED: &str = "Failed to read line.";
const PARSE_FAILED: &str = "Failed to parse.";

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
