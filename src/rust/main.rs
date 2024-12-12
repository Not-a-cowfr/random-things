use std::future::Future;
use std::pin::Pin;

use tokio::runtime::Runtime;

use crate::stuff::input;

mod mc_renderer;
mod paragraph_guesser;
mod stuff;
mod type_speedtest;
mod wordle;

pub fn main() {
	// display name, function
	let modules: Vec<(&str, fn())> = vec![
		("Paragraph Guesser", paragraph_guesser::start),
		("Minecraft Text Renderer", mc_renderer::start),
		("Typing Speed Test", type_speedtest::start),
	];

	#[allow(clippy::type_complexity)]
	let async_modules: Vec<(&str, fn() -> Pin<Box<dyn Future<Output = ()> + Send>>)> =
		vec![("Wordle", || Box::pin(wordle::start()))];

	loop {
		println!("\nSelect a module to run:");
		for (i, (name, _)) in modules.iter().enumerate() {
			println!("[{}] {}", i + 1, name);
		}
		for (i, (name, _)) in async_modules.iter().enumerate() {
			println!("[{}] {}", i + 1 + modules.len(), name);
		}

		let input = input("", false);

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
