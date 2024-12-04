/*
    * A rust remake of something i mdea many years ago in python
    * would be cool to make something that takes multiple paragraphs of input, and sorts the char_list based on real data of character frequency
 */

use std::io;
use std::io::Write;
use std::time::Instant;
use rand::Rng;

const SHOW_PROGRESS: bool = false;

fn to_string(vec:Vec<char>) -> String{
    return vec.iter().collect::<String>();
}

fn get_word() -> String {
    println!("Please enter a word: ");
    io::stdout().flush().unwrap();
    let mut word = String::new();
    io::stdin().read_line(&mut word).unwrap();
    word.trim().to_string()
}

fn smart_guess(word: String, char_list: Vec<char>) -> Vec<char> {
    let mut guess: Vec<char> = Vec::new();
    for character in word.chars() {
        for char in &char_list {
            if SHOW_PROGRESS {println!("[smart guess] {}{}", to_string(guess.clone()), char);}
            if character == *char {
                guess.push(character);
                break;
            }
        }
    }
    guess
}


fn bogo_guess(word: String, char_list: Vec<char>) -> Vec<char> {
    let mut guess: Vec<char> = Vec::new();
    for character in word.chars() {
        let mut random_char: char = ' ';
        while character != random_char {
            let random_index = rand::thread_rng().gen_range(0..char_list.len());
            random_char = char_list[random_index];
            if SHOW_PROGRESS {println!("[bogo guess] {}{}", to_string(guess.clone()), random_char);}
        }
        guess.push(random_char);
    }
    guess
}

pub(crate) fn main() {

    // sorted in order of most commonly used in text
    let char_list = vec![
        ' ',
        'e', 'E', 'a', 'A', 'o', 'O', 'i', 'I', 'u', 'U',
        't', 'T', 'n', 'N', 's', 'S', 'h', 'H', 'r', 'R', 'd', 'D', 'l', 'L', 'c', 'C', 'm', 'M',
        '.', ',', '!', '?',
        'w', 'W', 'f', 'F', 'g', 'G', 'y', 'Y', 'p', 'P', 'b', 'B', 'v', 'V', 'k', 'K', 'x', 'X', 'j', 'J', 'q', 'Q', 'z', 'Z',
        '\'', '"', '-', ':', ';', '(', ')', '[', ']', '{', '}', '_', '+', '=', '@', '#', '$', '%', '^', '&', '*', '/',
        '1', '0', '2', '3', '4', '5', '6', '7', '8', '9',
        '<', '>', '|', '\\', '`', '~'
    ];

    let word = get_word();

    let mut start = Instant::now();
    bogo_guess(word.clone(), char_list.clone());
    let bogo_time = start.elapsed();
    println!();

    start = Instant::now();
    smart_guess(word.clone(), char_list.clone());
    let smart_time = start.elapsed();

    println!("\nBogo Guess finished in: {:?}", bogo_time);
    println!("Smart Guess finished in: {:?}", smart_time);
}