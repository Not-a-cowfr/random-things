use std::collections::HashMap;
use std::io;

use image::{Rgb, RgbImage, open};
use rusttype::{Font, Scale, point};

fn render_text(
	text: &str,
	font: &Font,
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
						draw_character('&', &font, image, &mut x, y, scale, bold, current_color);
						continue;
					},
					| _ => {
						draw_character('\\', &font, image, &mut x, y, scale, bold, current_color);
						draw_character(
							next_char,
							&font,
							image,
							&mut x,
							y,
							scale,
							bold,
							current_color,
						);
						continue;
					},
				}
			} else {
				draw_character('\\', &font, image, &mut x, y, scale, bold, current_color);
				continue;
			}
		} else if c == '&' || c == '§' {
			if let Some(format_code) = chars.next() {
				match format_code {
					| 'l' => bold = true,
					| 'r' => {
						bold = false;
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

		draw_character(c, &font, image, &mut x, y, scale, bold, current_color);
	}

	image.save("output.png").expect("Failed to save image");
	image.clone()
}

fn draw_character(
	c: char,
	font: &Font,
	image: &mut RgbImage,
	x: &mut f32,
	y: f32,
	scale: Scale,
	bold: bool,
	color: Rgb<u8>,
) {
	let glyph = font.glyph(c);
	let scaled_glyph = glyph.scaled(if bold { Scale::uniform(20.0) } else { scale });
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
	}

	*x += scaled_glyph.h_metrics().advance_width;
}

pub fn main() {
	let font_data = include_bytes!("../assets/minecraft.ttf");
	let font = Font::try_from_bytes(font_data as &[u8]).expect("Error loading font");

	let mut image = open("assets\\background.png")
		.expect("Failed to load background image")
		.to_rgb8();

	let scale = Scale::uniform(16.0);

	println!("\nEnter text to render:");
	let mut text = String::new();
	io::stdin().read_line(&mut text).unwrap();
	let text = text.trim().to_string();

	render_text(&text, &font, &mut image, scale);
}

#[cfg(test)]
mod tests {
	use std::collections::HashMap;

	use image::{Rgb, RgbImage};

	use super::*;

	fn create_test_image() -> RgbImage { RgbImage::new(100, 100) }

	fn create_test_font() -> Font<'static> {
		let font_data = include_bytes!("../assets/minecraft.ttf");
		Font::try_from_bytes(font_data as &[u8]).expect("Error loading font")
	}

	#[test]
	fn renders_text_correctly() {
		let font = create_test_font();
		let mut image = create_test_image();
		let text = "Hello, world!";
		let scale = Scale::uniform(16.0);

		let image = render_text(&text, &font, &mut image, scale);

		assert_eq!(image.get_pixel(11, 40), &Rgb([255, 255, 255]));
	}

	#[test]
	fn handles_empty_text() {
		let font = create_test_font();
		let mut image = create_test_image();
		let text = "";
		let scale = Scale::uniform(16.0);

		let image = render_text(&text, &font, &mut image, scale);

		assert_eq!(image.get_pixel(11, 40), &Rgb([0, 0, 0]));
	}

	#[test]
	fn handles_formatting_codes() {
		let font = create_test_font();
		let mut image = create_test_image();
		let text = "§eHello";
		let scale = Scale::uniform(16.0);

		let image = render_text(&text, &font, &mut image, scale);

		assert_eq!(image.get_pixel(11, 40), &Rgb([255, 255, 85]));
	}

	#[test]
	fn handles_newline_character() {
		let font = create_test_font();
		let mut image = create_test_image();
		let text = "Hello\\nWorld";
		let scale = Scale::uniform(16.0);

		let image = render_text(&text, &font, &mut image, scale);

		assert_eq!(image.get_pixel(11, 40), &Rgb([255, 255, 255]));
		assert_eq!(
			image.get_pixel(11, 40 + (scale.y * 1.15) as u32),
			&Rgb([255, 255, 255])
		);
	}
}
