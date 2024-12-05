use std::io;

mod paragraph_guesser;

fn main() {
	// displayname, function
	let modules: Vec<(&str, fn())> = vec![("Paragraph Guesser", paragraph_guesser::main)];

	loop {
		println!("\nSelect a module to run:");
		for (i, (name, _)) in modules.iter().enumerate() {
			println!("{}: {}", i + 1, name);
		}

		let mut input = String::new();
		io::stdin()
			.read_line(&mut input)
			.expect("Failed to read line");

		if let Ok(choice) = input.trim().parse::<usize>() {
			if choice > 0 && choice <= modules.len() {
				modules[choice - 1].1();
				break;
			} else {
				println!("Invalid choice, please try again.");
			}
		} else {
			println!("Invalid input, please enter a number.");
		}
	}
}
