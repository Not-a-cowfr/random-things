use std::io;
use std::io::Write;
use std::time::Instant;

use rand::Rng;

const SHOW_PROGRESS: bool = true; // show progress of guesses (hurts performance of guessing, like up to 1,000x slower)

// sorted in order of most commonly used in text
pub(crate) static CHAR_LIST: &[char] = &[
	' ', 'e', 'E', 'a', 'A', 'o', 'O', 'i', 'I', 'u', 'U', 't', 'T', 'n', 'N', 's', 'S', 'h', 'H',
	'r', 'R', 'd', 'D', 'l', 'L', 'c', 'C', 'm', 'M', '.', ',', '!', '?', 'w', 'W', 'f', 'F', 'g',
	'G', 'y', 'Y', 'p', 'P', 'b', 'B', 'v', 'V', 'k', 'K', 'x', 'X', 'j', 'J', 'q', 'Q', 'z', 'Z',
	'\'', '"', '-', ':', ';', '(', ')', '[', ']', '{', '}', '_', '+', '=', '@', '#', '$', '%', '^',
	'&', '*', '/', '1', '0', '2', '3', '4', '5', '6', '7', '8', '9', '<', '>', '|', '\\', '`', '~',
];

fn to_string(vec: Vec<char>) -> String { vec.iter().collect::<String>() }

fn get_word() -> String {
	println!("\nPlease enter a word: ");
	io::stdout().flush().unwrap();
	let mut word = String::new();
	io::stdin().read_line(&mut word).unwrap();
	word.trim().to_string()
}

fn smart_guess(
	word: String,
	local_char_list: &[char],
) -> Vec<char> {
	let mut guess: Vec<char> = Vec::new();
	let print_progress = if SHOW_PROGRESS {
		Some(|guess: &Vec<char>, char: &char| {
			println!("[smart guess]\t{}{}", to_string(guess.clone()), char)
		})
	} else {
		None
	};

	for character in word.chars() {
		for char in local_char_list {
			if let Some(print) = &print_progress {
				print(&guess, char);
			}
			if character == *char {
				guess.push(character);
				break;
			}
		}
	}
	guess
}

fn bogo_guess(
	word: String,
	local_char_list: &[char],
) -> Vec<char> {
	let mut guess: Vec<char> = Vec::new();
	let print_progress = if SHOW_PROGRESS {
		Some(|guess: &Vec<char>, char: &char| {
			println!("[bogo guess]\t{}{}", to_string(guess.clone()), char)
		})
	} else {
		None
	};

	for character in word.chars() {
		let mut random_char: char = ' ';
		while character != random_char {
			let random_index = rand::thread_rng().gen_range(0..local_char_list.len());
			random_char = local_char_list[random_index];
			if let Some(print) = &print_progress {
				print(&guess, &random_char);
			}
		}
		guess.push(random_char);
	}
	guess
}

pub fn main() {
	let word = get_word();

	let mut start = Instant::now();
	bogo_guess(word.clone(), CHAR_LIST);
	let bogo_time = start.elapsed();

	start = Instant::now();
	smart_guess(word.clone(), CHAR_LIST);
	let smart_time = start.elapsed();

	println!("\nBogo Guess finished in: {:?}", bogo_time);
	println!("Smart Guess finished in: {:?}", smart_time);
}

#[cfg(test)]
mod tests {
	use rand::seq::IndexedRandom;
	use rand::thread_rng;

	use super::*;

	fn generate_random_string(
		length: usize,
		char_list: &[char],
	) -> String {
		let mut rng = thread_rng();
		(0..length)
			.map(|_| *char_list.choose(&mut rng).unwrap())
			.collect()
	}

	#[test]
	fn smart_guess_correct() {
		let word = generate_random_string(200, CHAR_LIST);
		let guessed = smart_guess(word.clone(), CHAR_LIST);
		assert_eq!(guessed, word.chars().collect::<Vec<char>>());
	}

	#[test]
	fn bogo_guess_correct() {
		let word = generate_random_string(200, CHAR_LIST);
		let guessed = bogo_guess(word.clone(), CHAR_LIST);
		assert_eq!(guessed, word.chars().collect::<Vec<char>>());
	}

	#[test]
	fn smart_guess_handles_empty_word() {
		let word = "".to_string();
		let guessed = smart_guess(word.clone(), CHAR_LIST);
		assert_eq!(guessed, word.chars().collect::<Vec<char>>());
	}

	#[test]
	fn bogo_guess_handles_empty_word() {
		let word = "".to_string();
		let guessed = bogo_guess(word.clone(), CHAR_LIST);
		assert_eq!(guessed, word.chars().collect::<Vec<char>>());
	}
}
