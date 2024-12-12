use std::io::{self, Write};

pub fn input(
	prompt: &str,
	newline: bool,
) -> String {
	if newline {
		println!("{}", prompt);
	} else {
		print!("{}", prompt);
	}

	io::stdout().flush().unwrap();
	let mut input = String::new();
	io::stdin().read_line(&mut input).unwrap();
	input.trim().to_string()
}

pub fn menu(options: Vec<&str>) -> u8 {
	println!();
	for (i, option) in options.iter().enumerate() {
		println!("[{}] {}", i + 1, option);
	}

	let mut choice: u8 = 0;
	let mut inputted_choice: String;
	while !(choice <= options.len() as u8 && choice > 0) {
		inputted_choice = input("", false);
		match inputted_choice.parse::<u8>() {
			| Ok(parsed) => {
				choice = parsed;
				if choice <= options.len() as u8 && choice > 0 {
					break;
				} else {
					println!("\nInvalid choice, please try again.");
				}
			},
			| Err(_) => println!("\nInvalid input, please enter a choice from the list."),
		}
	}

	choice
}
