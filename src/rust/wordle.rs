use std::io;
use std::io::Write;

use crate::main;
use crate::stuff::input;

async fn get_word(length: u8) -> String {
	let url = format!(
		"https://random-word-api.herokuapp.com/word?number=1&length={}",
		length
	);
	let response = reqwest::get(&url).await.unwrap();
	let word: Vec<String> = response.json().await.unwrap();
	word[0].clone()
}

fn process_guess(
	word: &str,
	guess: &str,
	guess_count: &mut u16,
) -> String {
	let mut remaining_letters: Vec<Option<char>> = word.chars().map(Some).collect();
	let mut correct = vec![None; word.len()];

	if guess.len() != word.len() {
		return format!(
			"\x1b[31mGuess must be \x1b[34m{}\x1b[31m letters long.\x1b[0m",
			word.len()
		);
	}

	*guess_count += 1;

	// First pass: mark correct positions
	for (i, guess_char) in guess.chars().enumerate() {
		if guess_char == word.chars().nth(i).unwrap() {
			correct[i] = Some(format!("\x1b[32m{}\x1b[0m", guess_char));
			remaining_letters[i] = None;
		}
	}

	// Second pass: mark present but incorrect positions
	for (i, guess_char) in guess.chars().enumerate() {
		if correct[i].is_none() {
			if let Some(pos) = remaining_letters
				.iter()
				.position(|&c| c == Some(guess_char))
			{
				correct[i] = Some(format!("\x1b[33m{}\x1b[0m", guess_char));
				remaining_letters[pos] = None;
			} else {
				correct[i] = Some(format!("\x1b[90m{}\x1b[0m", guess_char));
			}
		}
	}

	correct
		.iter()
		.map(|c| c.as_ref().unwrap().as_str())
		.collect::<String>()
}

pub async fn start() {
	let word: String = get_word(5).await;
	let mut guess_count: u16 = 0;

	print!("\nEnter your guess:");
	loop {
		let guess = input("", true);

		print!("\x1b[1A\x1b[2K");
		io::stdout().flush().unwrap();

		let result = process_guess(&word, &guess, &mut guess_count);
		print!("{}", result);

		if guess == word {
			println!(
				"\n\nCongratulations! You guessed the word in {} attempts",
				guess_count
			);
			break;
		}
	}

	main()
}

#[cfg(test)]
mod tests {
	use tokio::runtime::Runtime;

	use super::*;

	#[test]
	fn returns_word_in_correct_order() {
		let rt = Runtime::new().unwrap();
		let word = rt.block_on(get_word(5));
		assert_eq!(word.len(), 5);
	}

	#[test]
	fn correctly_colors_correct_positions() {
		let word = "apple".to_string();
		let guess = "apple".to_string();
		let mut guess_count = 0;
		let result = process_guess(&word, &guess, &mut guess_count);
		assert_eq!(
			result,
			"\x1b[32ma\x1b[0m\x1b[32mp\x1b[0m\x1b[32mp\x1b[0m\x1b[32ml\x1b[0m\x1b[32me\x1b[0m"
		);
		assert_eq!(guess_count, 1);
	}

	#[test]
	fn correctly_colors_present_but_incorrect_positions() {
		let word = "apple".to_string();
		let guess = "pleap".to_string();
		let mut guess_count = 0;
		let result = process_guess(&word, &guess, &mut guess_count);
		assert_eq!(
			result,
			"\x1b[33mp\x1b[0m\x1b[33ml\x1b[0m\x1b[33me\x1b[0m\x1b[33ma\x1b[0m\x1b[33mp\x1b[0m"
		);
		assert_eq!(guess_count, 1);
	}

	#[test]
	fn correctly_colors_incorrect_positions() {
		let word = "apple".to_string();
		let guess = "zzzzz".to_string();
		let mut guess_count = 0;
		let result = process_guess(&word, &guess, &mut guess_count);
		assert_eq!(
			result,
			"\x1b[90mz\x1b[0m\x1b[90mz\x1b[0m\x1b[90mz\x1b[0m\x1b[90mz\x1b[0m\x1b[90mz\x1b[0m"
		);
		assert_eq!(guess_count, 1);
	}

	#[test]
	fn guess_must_be_correct_length() {
		let word = "apple".to_string();
		let guess = "app".to_string();
		let mut guess_count = 0;
		let result = process_guess(&word, &guess, &mut guess_count);
		assert_eq!(
			result,
			"\x1b[31mGuess must be \x1b[34m5\x1b[31m letters long.\x1b[0m"
		);
		assert_eq!(guess_count, 0);
	}

	#[test]
	fn check_guess_count() {
		let word = "apple".to_string();
		let mut guess_count = 0;

		let guess = "app".to_string();
		let _ = process_guess(&word, &guess, &mut guess_count);
		assert_eq!(guess_count, 0);

		let guess = "apples".to_string();
		let _ = process_guess(&word, &guess, &mut guess_count);
		assert_eq!(guess_count, 0);

		let guess = "pleap".to_string();
		let _ = process_guess(&word, &guess, &mut guess_count);
		assert_eq!(guess_count, 1);

		let guess = "zzzzz".to_string();
		let _ = process_guess(&word, &guess, &mut guess_count);
		assert_eq!(guess_count, 2);
	}
}
