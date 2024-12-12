use std::collections::HashMap;
use std::ops::Add;
use std::path::Path;
use std::time::Duration;
use std::{fs, thread};

use arboard::{Clipboard, ImageData};
use image::buffer::ConvertBuffer;
use image::{Rgb, RgbImage, RgbaImage, open};
use rusttype::{Font, Scale, point};

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
						draw_character(
							next_char,
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

	let save_type = menu(vec!["Save to clipboard", "Save as file"]);

	if save_type == 2 {
		let path = input("\nEnter the filename to save the image as:", true);
		save(image, &path.add(".png"), save_type);
	} else {
		save(image, "output.png", save_type);
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

pub fn main() {
	let font_data_regular = include_bytes!("../assets/MinecraftRegular.otf");
	let font_data_bold = include_bytes!("../assets/MinecraftBold.otf");
	let font_data_italic = include_bytes!("../assets/MinecraftItalic.otf");
	let font_data_bold_italic = include_bytes!("../assets/MinecraftBoldItalic.otf");

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

	let mut image = open("assets\\background.png")
		.expect("Failed to load background image")
		.to_rgb8();

	let scale = Scale::uniform(16.0);

	let text = input("\nEnter text to render:", true);

	render_text(&text, &fonts, &mut image, scale);
}
