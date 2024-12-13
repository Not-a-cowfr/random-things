use std::time::{Duration, Instant};

use rand::Rng;

use crate::main;
use crate::stuff::{input, menu};

// sorted in order of most commonly used in text
pub(crate) static CHAR_LIST: &[char] = &[
	' ', 'e', 'E', 'a', 'A', 'o', 'O', 'i', 'I', 'u', 'U', 't', 'T', 'n', 'N', 's', 'S', 'h', 'H',
	'r', 'R', 'd', 'D', 'l', 'L', 'c', 'C', 'm', 'M', '.', ',', '!', '?', 'w', 'W', 'f', 'F', 'g',
	'G', 'y', 'Y', 'p', 'P', 'b', 'B', 'v', 'V', 'k', 'K', 'x', 'X', 'j', 'J', 'q', 'Q', 'z', 'Z',
	'\'', '"', '-', ':', ';', '(', ')', '[', ']', '{', '}', '_', '+', '=', '@', '#', '$', '%', '^',
	'&', '*', '/', '1', '0', '2', '3', '4', '5', '6', '7', '8', '9', '<', '>', '|', '\\', '`', '~',
];

fn to_string(vec: Vec<char>) -> String { vec.iter().collect::<String>() }

fn smart_guess_with_progress(
	word: String,
	local_char_list: &[char],
	inefficient_timeout: u64,
	efficiency: u16,
) -> Vec<char> {
	let mut guess: Vec<char> = Vec::new();
	let start_time = Instant::now();
	let mut guess_count: usize = 0;

	for character in word.chars() {
		for char in local_char_list {
			if start_time.elapsed().as_secs() < inefficient_timeout {
				println!("[smart guess]\t{}{}", to_string(guess.clone()), char);
			} else if guess_count % efficiency as usize == 0 {
				println!("[smart guess]\t{}{}", to_string(guess.clone()), char);
			}
			if character == *char {
				guess.push(character);
				break;
			}
			guess_count += 1;
		}
	}
	guess
}

fn smart_guess_without_progress(
	word: String,
	local_char_list: &[char],
) -> Vec<char> {
	let mut guess: Vec<char> = Vec::new();

	for character in word.chars() {
		for char in local_char_list {
			if character == *char {
				guess.push(character);
				break;
			}
		}
	}
	guess
}

fn bogo_guess_with_progress(
	word: String,
	local_char_list: &[char],
	inefficient_timeout: u64,
	efficiency: u16,
) -> Vec<char> {
	let mut guess: Vec<char> = Vec::new();
	let start_time = Instant::now();
	let mut guess_count: usize = 0;

	for character in word.chars() {
		let mut random_char: char = ' ';
		while character != random_char {
			let random_index = rand::thread_rng().gen_range(0..local_char_list.len());
			random_char = local_char_list[random_index];

			if start_time.elapsed().as_secs() < inefficient_timeout {
				println!("[bogo guess]\t{}{}", to_string(guess.clone()), random_char);
			} else if guess_count % efficiency as usize == 0 {
				println!("[bogo guess]\t{}{}", to_string(guess.clone()), random_char);
			}
			guess_count += 1;
		}
		guess.push(random_char);
	}
	guess
}

fn bogo_guess_without_progress(
	word: String,
	local_char_list: &[char],
) -> Vec<char> {
	let mut guess: Vec<char> = Vec::new();

	for character in word.chars() {
		let mut random_char: char = ' ';
		while character != random_char {
			let random_index = rand::thread_rng().gen_range(0..local_char_list.len());
			random_char = local_char_list[random_index];
		}
		guess.push(random_char);
	}
	guess
}

pub fn start() {
	let show_progress: bool; // show progress of guesses (hurts performance of guessing, like up to 1,000x slower)
	let word = input("\nEnter a phrase:", true);

	let save_type = menu(vec!["Show Progress (slow)", "Hide progress (fast)"]);
	match save_type {
		| 1 => show_progress = true,
		| 2 => show_progress = false,
		| _ => show_progress = true,
	}

	let bogo_time: Duration;
	let smart_time: Duration;
	if show_progress {
		let start_efficient_mode: u64;
		let start_efficient_mode_input = input(
			"\nHow long should it show full progress before switching to a more efficient method (won't show every single step)?",
			true,
		);
		match start_efficient_mode_input.parse::<u64>() {
			| Ok(parsed) => start_efficient_mode = parsed,
			| Err(_) => {
				println!("Invalid choice! Defaulting to 10 seconds");
				start_efficient_mode = 10;
			},
		}

		let efficiency: u16;
		let efficiency_input = input(
			"\nHow many progress checks it skips after enabling efficiency mode:",
			true,
		);
		match efficiency_input.parse::<u16>() {
			| Ok(parsed) => efficiency = parsed,
			| Err(_) => {
				println!("Invalid Choice! Defaulting to 10 lines!");
				efficiency = 10;
			},
		}

		let mut start = Instant::now();
		bogo_guess_with_progress(word.clone(), CHAR_LIST, start_efficient_mode, efficiency);
		bogo_time = start.elapsed();
		start = Instant::now();
		smart_guess_with_progress(word.clone(), CHAR_LIST, start_efficient_mode, efficiency);
		smart_time = start.elapsed();
	} else {
		let mut start = Instant::now();
		bogo_guess_without_progress(word.clone(), CHAR_LIST);
		bogo_time = start.elapsed();
		start = Instant::now();
		smart_guess_without_progress(word.clone(), CHAR_LIST);
		smart_time = start.elapsed();
	}

	println!("\nBogo Guess finished in: {:?}", bogo_time);
	println!("Smart Guess finished in: {:?}", smart_time);

	main()
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
		let guessed = smart_guess_without_progress(word.clone(), CHAR_LIST);
		assert_eq!(guessed, word.chars().collect::<Vec<char>>());
	}

	#[test]
	fn bogo_guess_correct() {
		let word = generate_random_string(200, CHAR_LIST);
		let guessed = bogo_guess_without_progress(word.clone(), CHAR_LIST);
		assert_eq!(guessed, word.chars().collect::<Vec<char>>());
	}

	#[test]
	fn smart_guess_handles_empty_word() {
		let word = "".to_string();
		let guessed = smart_guess_without_progress(word.clone(), CHAR_LIST);
		assert_eq!(guessed, word.chars().collect::<Vec<char>>());
	}

	#[test]
	fn bogo_guess_handles_empty_word() {
		let word = "".to_string();
		let guessed = bogo_guess_without_progress(word.clone(), CHAR_LIST);
		assert_eq!(guessed, word.chars().collect::<Vec<char>>());
	}
}
