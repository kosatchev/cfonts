use crate::config::Options;
use crate::debug::{d, Dt};

pub fn get_letter_space(letter_space: &[String], options: &Options) -> Vec<String> {
	d("chars::get_letter_space()", 2, Dt::Head, options, &mut std::io::stdout());
	d(
		&format!(
			"chars::get_letter_space()\nletter_space:{:?}\noptions.letter_spacing:{:?}",
			letter_space, options.letter_spacing
		),
		2,
		Dt::Log,
		options,
		&mut std::io::stdout(),
	);

	let mut output = Vec::new();

	for letter_space_line in letter_space {
		let space = match letter_space_line.len() {
			0 => String::from(" ").repeat(options.letter_spacing as usize),
			_ => letter_space_line.repeat(options.letter_spacing as usize),
		};

		output.push(space);
	}

	d(&format!("chars::get_letter_space() -> {:?}", output), 2, Dt::Log, options, &mut std::io::stdout());
	output
}

// pub fn get_longest_line_len(output: &Vec<String>, font_lines: &usize, options: &Options) -> usize {}

// pub fn get_char_length(char: &Vec<String>, options: &Options) -> usize {}

// pub fn paint_char(char: &Vec<String>, colors: &Vec<[String; 2]>, options: &Options) -> Vec<String> {}

// pub fn add_char(char: &Vec<String>, output: &Vec<String>, options: &Options) -> Vec<String> {}

pub fn add_line(output: &mut Vec<String>, font_lines: usize, options: &Options) -> Vec<String> {
	d("chars::add_line()", 2, Dt::Head, options, &mut std::io::stdout());
	d(
		&format!("chars::add_line()\noutput:{:?}\nfont_lines:{:?}", output, font_lines),
		2,
		Dt::Log,
		options,
		&mut std::io::stdout(),
	);

	for _ in 0..font_lines {
		output.push(String::new());
	}

	d(&format!("chars::add_line() -> {:?}", output), 2, Dt::Log, options, &mut std::io::stdout());
	output.to_vec()
}

// pub fn align_last_line(output: &Vec<String>, font_lines: &usize, options: &Options) -> Vec<String> {}
