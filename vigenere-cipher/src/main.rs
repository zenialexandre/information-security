use std::io;
use std::io::Write;

const FLUSH_FAILED: &str = "Failed to flush.";
const READ_LINE_FAILED: &str = "Failed to read line.";
const PARSE_FAILED: &str = "Failed to parse.";
const ALPHABET_MAXIMUM_INDEX: usize = 26;
const ALPHABET_MINIMUM_INDEX: usize = 0;

fn main() {
    initialize_program();
}

fn initialize_program() {
    println!("\nVigenere Cipher");
    println!("#######################################");

    let user_process_choice: usize = get_user_process_choice();

    if user_process_choice != 1 && user_process_choice != 2 {
        panic!("Wrong informed choice. The choice must be equal to 1 or 2.");
    } else if user_process_choice == 1 || user_process_choice == 2 {
        execute_process_based_on_user_choice(
            user_process_choice,
            get_user_simple_phrase().to_ascii_uppercase(),
            get_user_security_key().to_ascii_uppercase()
        );     
    }
}

fn get_user_process_choice() -> usize {
    let mut user_process_choice_input: String = String::new();
    
    println!("\nPlease inform 1 for ciphering a phrase or 2 for deciphering a phrase: ");
    io::stdout().flush().expect(FLUSH_FAILED);
    io::stdin().read_line(&mut user_process_choice_input).expect(READ_LINE_FAILED);
    return user_process_choice_input.trim().parse().expect(PARSE_FAILED);
}

fn execute_process_based_on_user_choice(
    user_process_choice: usize,
    user_simple_phrase: String,
    user_security_key: String
) {
    let alphabet: String = get_alphabet();
    let user_security_key_indexes: Vec<usize> = get_user_security_key_indexes(
        user_security_key,
        alphabet.clone()
    );

    if user_process_choice == 1 {
        println!("\nInitializing the ciphering process...");
        execute_ciphering_process(
            alphabet,
            user_security_key_indexes,
            user_simple_phrase
        );
    } else if user_process_choice == 2 {
        println!("\nInitializing the deciphering process...");
        execute_deciphering_process(
            alphabet,
            user_security_key_indexes,
            user_simple_phrase
        );
    }
}

fn get_user_simple_phrase() -> String {
    let mut user_simple_phrase_input: String = String::new();

    println!("\nPlease inform the phrase to be used: ");
    io::stdout().flush().expect(FLUSH_FAILED);
    io::stdin().read_line(&mut user_simple_phrase_input).expect(READ_LINE_FAILED);
    return user_simple_phrase_input;
}

fn get_user_security_key() -> String {
    let mut user_security_key_input: String = String::new();

    println!("\nPlease inform the key to be used: ");
    io::stdout().flush().expect(FLUSH_FAILED);
    io::stdin().read_line(&mut user_security_key_input).expect(READ_LINE_FAILED);
    return user_security_key_input;
}

fn execute_ciphering_process(
    alphabet: String,
    user_security_key_indexes: Vec<usize>,
    user_simple_phrase: String
) {
    println!("\nCiphered phrase: {}", get_processed_phrase(
        alphabet,
        user_security_key_indexes,
        user_simple_phrase,
        true
    ));
}

fn execute_deciphering_process(
    alphabet: String,
    user_security_key_indexes: Vec<usize>,
    user_simple_phrase: String
) {
    println!("\nDeciphered phrase: {}", get_processed_phrase(
        alphabet,
        user_security_key_indexes,
        user_simple_phrase,
        false
    ));
}

fn get_alphabet() -> String {
    let mut alphabet: String = String::new();

    for character in 'A'..='Z' {
        alphabet.push(character);
    }
    return alphabet;
}

fn get_user_security_key_indexes(
    user_security_key: String,
    alphabet: String
) -> Vec<usize> {
    let mut user_security_key_indexes: Vec<usize> = Vec::new();
    let mut user_security_key_character_index: Option<usize>;

    for user_security_key_character in user_security_key.chars() {
        user_security_key_character_index = alphabet.find(user_security_key_character);

        match user_security_key_character_index {
            Some(character_index) => { user_security_key_indexes.push(character_index); },
            None => {}
        };
    }
    return user_security_key_indexes;
}

fn get_processed_phrase(
    alphabet: String,
    user_security_key_indexes: Vec<usize>,
    user_simple_phrase: String,
    is_ciphering: bool
) -> String {
    let mut user_simple_phrase_processed: String = String::new();
    let mut alphabet_character_index: Option<usize>;
    let mut user_security_key_index_flag: usize = 0;

    for character in user_simple_phrase.chars() {
        if character != ' ' {
            alphabet_character_index = alphabet.find(character);
    
            match alphabet_character_index {
                Some(index) => {
                    user_simple_phrase_processed.push(get_character_from_indexes(
                        alphabet.clone(),
                        index,
                        user_security_key_indexes.clone(),
                        user_security_key_index_flag,
                        is_ciphering
                    ));
            
                    if user_security_key_index_flag + 1 < user_security_key_indexes.len() {
                        user_security_key_index_flag += 1;
                    } else {
                        user_security_key_index_flag = 0;
                    }
                },
                None => {}
            };
        } else {
            user_simple_phrase_processed.push(character);
        }
    }
    return user_simple_phrase_processed;
}

fn get_character_from_indexes(
    alphabet: String,
    mut alphabet_character_index: usize,
    user_security_key_indexes: Vec<usize>,
    user_security_key_index_flag: usize,
    is_ciphering: bool
) -> char {
    if is_ciphering {
        alphabet_character_index += user_security_key_indexes[user_security_key_index_flag];

        if alphabet_character_index >= ALPHABET_MAXIMUM_INDEX {
            alphabet_character_index -= ALPHABET_MAXIMUM_INDEX;
        }
    } else {
        let mut alphabet_character_index_i32: i32 = get_converted_i32_from_usize(alphabet_character_index);
        alphabet_character_index_i32 -= user_security_key_indexes[user_security_key_index_flag] as i32;

        if alphabet_character_index_i32 < ALPHABET_MINIMUM_INDEX as i32 {
            alphabet_character_index_i32 += ALPHABET_MAXIMUM_INDEX as i32;
            alphabet_character_index = alphabet_character_index_i32 as usize;
        } else {
            alphabet_character_index = alphabet_character_index_i32 as usize;
        }
    }
    return alphabet.chars().nth(alphabet_character_index).unwrap();
}

fn get_converted_i32_from_usize(index: usize) -> i32 {
    return index as i32;
}
