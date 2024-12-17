use std::collections::HashMap;
use std::io::{Write, stdout};
use std::ops::Add;
use std::path::Path;
use std::time::Duration;
use std::{fs, thread};

use arboard::{Clipboard, ImageData};
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use image::buffer::ConvertBuffer;
use image::{GenericImage, Rgb, RgbImage, RgbaImage, open};
use minifb::{Window, WindowOptions};
use rusttype::{Font, Scale, point};

use crate::main;
use crate::stuff::{input, menu};

fn save(
	image: &RgbImage,
	path: &str,
	save_type: u8,
) {
	match save_type {
		| 1 => save_image_to_clipboard(image),
		| 2 => save_image_to_file(image, path),
		| _ => {},
	}
}

fn save_image_to_clipboard(image: &RgbImage) {
	let rgba_image: RgbaImage = image.convert();
	let width = rgba_image.width() as usize;
	let height = rgba_image.height() as usize;
	let bytes = rgba_image.into_raw();
	let image_data = ImageData {
		width,
		height,
		bytes: std::borrow::Cow::Owned(bytes),
	};

	let mut retries = 5;
	while retries > 0 {
		let clipboard = Clipboard::new();
		if let Ok(mut clipboard) = clipboard {
			if clipboard.set_image(image_data.clone()).is_ok() {
				println!("\n\x1b[32mSuccess!\x1b[0m Image copied to clipboard");
				break;
			}
		}
		retries -= 1;
		thread::sleep(Duration::from_millis(100));
	}
}

fn save_image_to_file(
	image: &RgbImage,
	path: &str,
) {
	fs::create_dir_all("output").expect("Failed to create output directory");
	let full_path = format!("output\\{}", path);
	image.save(&full_path).expect("Failed to save image");
	let absolute_path =
		fs::canonicalize(Path::new(&full_path)).expect("Failed to get absolute path of image");
	let display_path = absolute_path
		.strip_prefix(r"\\?\")
		.unwrap_or(&absolute_path);
	println!(
		"\n\x1b[32mSuccess!\x1b[0m File saved at: {}",
		display_path.display()
	);
}

fn render_text(
	text: &str,
	fonts: &HashMap<&str, Font>,
	image: &mut RgbImage,
	scale: Scale,
) -> RgbImage {
	let colors: HashMap<char, Rgb<u8>> = [
		('0', Rgb([0, 0, 0])),       // black
		('1', Rgb([0, 0, 170])),     // dark blue
		('2', Rgb([0, 170, 0])),     // dark green
		('3', Rgb([0, 170, 170])),   // dark aqua
		('4', Rgb([170, 0, 0])),     // dark red
		('5', Rgb([170, 0, 170])),   // dark purple
		('6', Rgb([255, 170, 0])),   // gold
		('7', Rgb([170, 170, 170])), // gray
		('8', Rgb([85, 85, 85])),    // dark gray
		('9', Rgb([85, 85, 255])),   // blue
		('a', Rgb([85, 255, 85])),   // green
		('b', Rgb([85, 255, 255])),  // aqua
		('c', Rgb([255, 85, 85])),   // red
		('d', Rgb([255, 85, 255])),  // light purple
		('e', Rgb([255, 255, 85])),  // yellow
		('f', Rgb([255, 255, 255])), // white
	]
	.iter()
	.cloned()
	.collect();

	let line_height = scale.y * 1.15;
	let mut x = 10.0;
	let mut y = 50.0;

	let mut current_color = Rgb([255, 255, 255]);
	let mut bold = false;
	let mut italic = false;
	let mut strikethrough = false;
	let mut underline = false;

	let mut chars = text.chars().peekable();

	while let Some(c) = chars.next() {
		if c == '\\' {
			if let Some(next_char) = chars.next() {
				match next_char {
					| 'n' => {
						x = 10.0;
						y += line_height;
						continue;
					},
					| '&' => {
						draw_character(
							'&',
							fonts,
							image,
							&mut x,
							y,
							scale,
							bold,
							italic,
							strikethrough,
							underline,
							current_color,
						);
						continue;
					},
					| _ => {
						draw_character(
							'\\',
							fonts,
							image,
							&mut x,
							y,
							scale,
							bold,
							italic,
							strikethrough,
							underline,
							current_color,
						);
						continue;
					},
				}
			} else {
				draw_character(
					'\\',
					fonts,
					image,
					&mut x,
					y,
					scale,
					bold,
					italic,
					strikethrough,
					underline,
					current_color,
				);
				continue;
			}
		} else if c == '&' || c == '§' {
			if let Some(format_code) = chars.next() {
				match format_code {
					| 'l' => bold = true,
					| 'o' => italic = true,
					| 'm' => strikethrough = true,
					| 'n' => underline = true,
					| 'r' => {
						bold = false;
						italic = false;
						strikethrough = false;
						underline = false;
						current_color = Rgb([255, 255, 255]);
					},
					| _ if colors.contains_key(&format_code) => {
						current_color = colors[&format_code];
					},
					| _ => {},
				}
				continue;
			}
		}

		draw_character(
			c,
			fonts,
			image,
			&mut x,
			y,
			scale,
			bold,
			italic,
			strikethrough,
			underline,
			current_color,
		);
	}
	image.clone()
}

fn draw_character(
	c: char,
	fonts: &HashMap<&str, Font>,
	image: &mut RgbImage,
	x: &mut f32,
	y: f32,
	scale: Scale,
	bold: bool,
	italic: bool,
	strikethrough: bool,
	underline: bool,
	color: Rgb<u8>,
) {
	let font_key = match (bold, italic) {
		| (true, true) => "bold_italic",
		| (true, false) => "bold",
		| (false, true) => "italic",
		| (false, false) => "regular",
	};

	let font = &fonts[font_key];
	let glyph = font.glyph(c);
	let scaled_glyph = glyph.scaled(scale);
	let positioned_glyph = scaled_glyph.clone().positioned(point(*x, y));

	if let Some(bounding_box) = positioned_glyph.pixel_bounding_box() {
		positioned_glyph.draw(|dx, dy, v| {
			let px = bounding_box.min.x + dx as i32;
			let py = bounding_box.min.y + dy as i32;

			if px >= 0 && py >= 0 && px < image.width() as i32 && py < image.height() as i32 {
				let px = px as u32;
				let py = py as u32;

				let bg_pixel = image.get_pixel(px, py);

				let alpha = v;
				let blended_color = Rgb([
					(bg_pixel[0] as f32 * (1.0 - alpha) + color[0] as f32 * alpha) as u8,
					(bg_pixel[1] as f32 * (1.0 - alpha) + color[1] as f32 * alpha) as u8,
					(bg_pixel[2] as f32 * (1.0 - alpha) + color[2] as f32 * alpha) as u8,
				]);

				image.put_pixel(px, py, blended_color);
			}
		});

		if strikethrough {
			let y_strike = y - scale.y / 3.0;
			for px in bounding_box.min.x..bounding_box.max.x {
				if px >= 0
					&& y_strike >= 0.0
					&& px < image.width() as i32
					&& y_strike < image.height() as f32
				{
					image.put_pixel(px as u32, y_strike as u32, color);
				}
			}
		}

		if underline {
			let y_underline = y + scale.y / 10.0;
			for px in bounding_box.min.x..bounding_box.max.x {
				if px >= 0
					&& y_underline >= 0.0
					&& px < image.width() as i32
					&& y_underline < image.height() as f32
				{
					image.put_pixel(px as u32, y_underline as u32, color);
				}
			}
		}
	}

	*x += scaled_glyph.h_metrics().advance_width;
}

pub fn start() {
	let current_dir = std::env::current_dir().expect("Failed to get current directory");
	println!("Checking for font files in directory: {:?}", current_dir);

	let font_data_regular = include_bytes!("assets/MinecraftRegular.otf");
	let font_data_bold = include_bytes!("assets/MinecraftBold.otf");
	let font_data_italic = include_bytes!("assets/MinecraftItalic.otf");
	let font_data_bold_italic = include_bytes!("assets/MinecraftBoldItalic.otf");

	let fonts = HashMap::from([
		(
			"regular",
			Font::try_from_bytes(font_data_regular as &[u8]).expect("Error loading regular font"),
		),
		(
			"bold",
			Font::try_from_bytes(font_data_bold as &[u8]).expect("Error loading bold font"),
		),
		(
			"italic",
			Font::try_from_bytes(font_data_italic as &[u8]).expect("Error loading italic font"),
		),
		(
			"bold_italic",
			Font::try_from_bytes(font_data_bold_italic as &[u8])
				.expect("Error loading bold italic font"),
		),
	]);

	let background_image = open("../assets/background.png")
		.expect("Failed to load background image")
		.to_rgb8();
	let (width, height) = background_image.dimensions();
	let mut image = background_image.clone();

	let scale = Scale::uniform(16.0);

	let mut text = String::new();
	let mut window = Window::new(
		"Text Renderer",
		width as usize,
		height as usize,
		WindowOptions::default(),
	)
	.unwrap_or_else(|e| {
		panic!("{}", e);
	});

	println!(
		"\x1b[1mHelp Menu:\x1b[0m\n\
        \nColor Codes:\n\
        \t\x1b[30m&0 or §0: Black\x1b[0m\n\
        \t\x1b[34m&1 or §1: Dark Blue\x1b[0m\n\
        \t\x1b[32m&2 or §2: Dark Green\x1b[0m\n\
        \t\x1b[36m&3 or §3: Dark Aqua\x1b[0m\n\
        \t\x1b[31m&4 or §4: Dark Red\x1b[0m\n\
        \t\x1b[35m&5 or §5: Dark Purple\x1b[0m\n\
        \t\x1b[33m&6 or §6: Gold\x1b[0m\n\
        \t\x1b[37m&7 or §7: Gray\x1b[0m\n\
        \t\x1b[90m&8 or §8: Dark Gray\x1b[0m\n\
        \t\x1b[94m&9 or §9: Blue\x1b[0m\n\
        \t\x1b[92m&a or §a: Green\x1b[0m\n\
        \t\x1b[96m&b or §b: Aqua\x1b[0m\n\
        \t\x1b[91m&c or §c: Red\x1b[0m\n\
        \t\x1b[95m&d or §d: Light Purple\x1b[0m\n\
        \t\x1b[93m&e or §e: Yellow\x1b[0m\n\
        \t\x1b[97m&f or §f: White\x1b[0m\n\
        \nFormatting Codes:\n\
        \t\x1b[1m&l or §l: Bold\x1b[0m\n\
        \t\x1b[3m&o or §o: Italic\x1b[0m\n\
        \t\x1b[9m&m or §m: Strikethrough\x1b[0m\n\
        \t\x1b[4m&n or §n: Underline\x1b[0m\n\
        \t&r or §r: Reset all formatting\n\
        \nSpecial Characters:\n\
        \t\\& for &\n\
        \t\\§ for §\n\
        \t\\\\ for \\\n\
        \t\\n for new line\n"
	);

	enable_raw_mode().expect("Failed to enable raw mode");

	loop {
		if event::poll(Duration::from_millis(100)).unwrap() {
			if let Event::Key(key_event) = event::read().unwrap() {
				if key_event.kind == KeyEventKind::Press {
					match key_event.code {
						| KeyCode::Char(c) => {
							text.push(c);
						},
						| KeyCode::Backspace => {
							text.pop();
						},
						| KeyCode::Enter => {
							break;
						},
						| _ => {},
					}

					print!("\r\x1b[2K{}", text);
					stdout().flush().unwrap();

					image
						.copy_from(&background_image, 0, 0)
						.expect("Failed to copy background image");

					let rendered_image = render_text(&text, &fonts, &mut image, scale);
					let buffer: Vec<u32> = rendered_image
						.pixels()
						.map(|p| {
							let [r, g, b] = p.0;
							((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
						})
						.collect();
					window
						.update_with_buffer(
							&buffer,
							image.width() as usize,
							image.height() as usize,
						)
						.unwrap();
				}
			}
		}
	}

	disable_raw_mode().expect("Failed to disable raw mode");

	println!();
	let save_type = menu(vec!["Save to clipboard", "Save as file"]);

	match save_type {
		| 1 => {
			save(&image, "", save_type);
		},
		| 2 => {
			let path = input("\nEnter the filename to save the image as:", true);
			save(&image, &path.add(".png"), save_type);
		},
		| _ => {},
	}

	main()
}
