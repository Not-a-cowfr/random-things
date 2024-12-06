use std::fs::File;
use std::io::{self, BufReader, Write};
use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use rand::seq::IndexedRandom;
use serde::Deserialize;

#[derive(Deserialize)]
struct Quote {
	text:   String,
	source: String,
	length: usize,
	id:     usize,
}

#[derive(Deserialize)]
struct Phrases {
	language: String,
	groups:   Vec<(usize, usize)>,
	quotes:   Vec<Quote>,
}

fn get_phrase() -> String {
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
	input_len: usize,
) -> f64 {
	let minutes = duration.as_secs_f64() / 60.0;
	(input_len as f64 / 5.0) / minutes
}

pub fn main() {
	enable_raw_mode().unwrap();
	let mut stdout = io::stdout();
	let predefined = get_phrase().chars().collect::<Vec<_>>();
	let mut input = String::new();
	let mut correct_count = 0;
	let mut incorrect_count = 0;
	let mut total_keystrokes = 0;

	write!(
		stdout,
		"\n\r\x1b[2K{}",
		predefined.iter().collect::<String>()
	)
	.unwrap();
	stdout.flush().unwrap();

	let start_time = Instant::now();

	loop {
		if event::poll(std::time::Duration::from_millis(500)).unwrap() {
			if let Event::Key(key_event) = event::read().unwrap() {
				if key_event.kind == KeyEventKind::Press {
					match key_event.code {
						| KeyCode::Char(c) => {
							input.push(c);
							process_input(
								&input,
								&predefined,
								&mut stdout,
								&mut correct_count,
								&mut incorrect_count,
								&mut total_keystrokes,
							);
						},
						| KeyCode::Backspace => {
							input.pop();
							process_input(
								&input,
								&predefined,
								&mut stdout,
								&mut correct_count,
								&mut incorrect_count,
								&mut total_keystrokes,
							);
						},
						| KeyCode::Esc => {
							break;
						},
						| _ => {},
					}
				}
			}
		}

		if input.len() >= predefined.len() {
			break;
		}
	}

	let duration = start_time.elapsed();
	let accuracy = calculate_accuracy(correct_count, total_keystrokes);
	let wpm = calculate_wpm(duration, input.len());

	disable_raw_mode().unwrap();

	println!("\nTime taken: {:.2?}", duration);
	println!("Accuracy: {:.2}%", accuracy);
	println!("WPM: {:.2}", wpm);
}
