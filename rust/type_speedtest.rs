use std::fs::File;
use std::io::{self, BufReader, Write};
use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use rand::seq::IndexedRandom;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize)]
struct Quote {
	text:   String,
	source: String,
	length: usize,
	id:     usize,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct Phrases {
	language: String,
	groups:   Vec<(usize, usize)>,
	quotes:   Vec<Quote>,
}

fn get_phrase() -> String {
	// TODO make api and use api to get phrases
	let file = File::open("assets/phrases.json").expect("Failed to open phrases.json");
	let reader = BufReader::new(file);
	let phrases: Phrases = serde_json::from_reader(reader).expect("Failed to parse JSON");

	let quote = phrases
		.quotes
		.choose(&mut rand::thread_rng())
		.expect("No quotes found");
	quote.text.clone()
}

fn process_input(
	input: &str,
	predefined: &[char],
	stdout: &mut io::Stdout,
	correct_count: &mut usize,
	incorrect_count: &mut usize,
	total_keystrokes: &mut usize,
) {
	let mut output = String::new();
	*correct_count = 0;
	*incorrect_count = 0;

	for (i, &pre_char) in predefined.iter().enumerate() {
		if i < input.len() {
			let input_char = input.chars().nth(i).unwrap();
			if input_char == pre_char {
				output.push_str(&format!("\x1b[32m{}\x1b[0m", input_char));
				*correct_count += 1;
			} else {
				output.push_str(&format!("\x1b[31m{}\x1b[0m", input_char));
				*incorrect_count += 1;
			}
		} else {
			output.push(pre_char);
		}
	}
	*total_keystrokes += 1;
	write!(stdout, "\r\x1b[2K{}", output).unwrap();
	stdout.flush().unwrap();
}

fn calculate_accuracy(
	correct_count: usize,
	total_keystrokes: usize,
) -> f64 {
	if total_keystrokes == 0 {
		0.0
	} else {
		(correct_count as f64 / total_keystrokes as f64) * 100.0
	}
}

fn calculate_wpm(
	duration: Duration,
	input: &str,
) -> (f64, usize) {
	let minutes = duration.as_secs_f64() / 60.0;
	let word_count = input.split_whitespace().count();
	let wpm = word_count as f64 / minutes;
	(wpm, word_count)
}

pub fn main() {
	enable_raw_mode().unwrap();
	let mut stdout = io::stdout();
	let phrase = get_phrase().chars().collect::<Vec<_>>();
	let mut input = String::new();
	let mut correct_count = 0;
	let mut incorrect_count = 0;
	let mut total_keystrokes = 0;

	write!(stdout, "\n\r\x1b[2K{}", phrase.iter().collect::<String>()).unwrap();
	stdout.flush().unwrap();

	let start_time = Instant::now();

	loop {
		if event::poll(Duration::from_millis(500)).unwrap() {
			if let Event::Key(key_event) = event::read().unwrap() {
				if key_event.kind == KeyEventKind::Press {
					match key_event.code {
						| KeyCode::Char(c) => {
							input.push(c);
							process_input(
								&input,
								&phrase,
								&mut stdout,
								&mut correct_count,
								&mut incorrect_count,
								&mut total_keystrokes,
							);
							stdout.flush().unwrap();
						},
						| KeyCode::Backspace => {
							input.pop();
							process_input(
								&input,
								&phrase,
								&mut stdout,
								&mut correct_count,
								&mut incorrect_count,
								&mut total_keystrokes,
							);
							stdout.flush().unwrap();
						},
						| KeyCode::Esc => {
							stdout.flush().unwrap();
							break;
						},
						| _ => {
							stdout.flush().unwrap();
						},
					}
				}
			}
		}

		if input.len() >= phrase.len() {
			break;
		}
	}

	let duration = start_time.elapsed();
	let accuracy = calculate_accuracy(correct_count, total_keystrokes);
	let (wpm, _word_count) = calculate_wpm(duration, &input);

	disable_raw_mode().unwrap();

	println!("\nTime taken: {:.2?}", duration);
	println!("Accuracy: {:.2}%", accuracy);
	println!("WPM: {:.2}", wpm);
}

#[cfg(test)]
mod tests {
	use std::time::Duration;

	use super::*;

	#[test]
	fn get_phrase_works() {
		let phrase = get_phrase();
		assert!(!phrase.is_empty());
	}

	#[test]
	fn check_accuracy() {
		let accuracy1 = calculate_accuracy(10, 20);
		let accuracy2 = calculate_accuracy(0, 100);
		let accuracy3 = calculate_accuracy(100, 100);
		assert_eq!(accuracy1, 50.0);
		assert_eq!(accuracy2, 0.0);
		assert_eq!(accuracy3, 100.0);
	}

	#[test]
	fn accuracy_handles_zero_input() {
		let correct_count = 0;
		let total_keystrokes = 0;
		let accuracy = calculate_accuracy(correct_count, total_keystrokes);
		assert_eq!(accuracy, 0.0);
	}

	#[test]
	fn check_wpm() {
		let duration = Duration::from_secs(60);
		let (wpm, _length) = calculate_wpm(duration, "This is a test sentence that has nine words");
		assert_eq!(wpm, 9.0);
	}

	#[test]
	fn wpm_handles_zero_duration() {
		let duration = Duration::from_secs(0);
		let (wpm, _length) =
			calculate_wpm(duration, "This is a test sentence that has ten words (wtf)");
		assert!(wpm.is_infinite());
	}

	#[test]
	fn check_wpm_wordcount() {
		let duration = Duration::from_secs(60);
		let (wpm, _length) = calculate_wpm(
			duration,
			"This is a test sentence that has TWELVE words (WTF IS HAPPENING)",
		);
		assert_eq!(wpm, 12.0);
	}
}
