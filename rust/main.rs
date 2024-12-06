use std::future::Future;
use std::io;
use std::pin::Pin;

use tokio::runtime::Runtime;

mod mc_renderer;
mod paragraph_guesser;
mod type_speedtest;
mod wordle;

pub fn main() {
	// displayname, function
	let modules: Vec<(&str, fn())> = vec![
		("Paragraph Guesser", paragraph_guesser::main),
		("Minecraft Text Renderer", mc_renderer::main),
		("Typing Speed Test", type_speedtest::main),
	];

	#[allow(clippy::type_complexity)]
	let async_modules: Vec<(&str, fn() -> Pin<Box<dyn Future<Output = ()> + Send>>)> =
		vec![("Wordle", || Box::pin(wordle::main()))];

	loop {
		println!("\nSelect a module to run:");
		for (i, (name, _)) in modules.iter().enumerate() {
			println!("[{}] {}", i + 1, name);
		}
		for (i, (name, _)) in async_modules.iter().enumerate() {
			println!("[{}] {}", i + 1 + modules.len(), name);
		}

		let mut input = String::new();
		io::stdin()
			.read_line(&mut input)
			.expect("Failed to read line");

		if let Ok(choice) = input.trim().parse::<usize>() {
			if choice > 0 && choice <= modules.len() {
				modules[choice - 1].1();
				break;
			} else if choice > modules.len() && choice <= modules.len() + async_modules.len() {
				let rt = Runtime::new().unwrap();
				rt.block_on(async_modules[choice - 1 - modules.len()].1());
				break;
			} else {
				println!("Invalid choice, please try again.");
			}
		} else {
			println!("Invalid input, please enter a number.");
		}
	}
}
