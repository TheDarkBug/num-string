use crate::interface::*;
use std::fs::File;
use std::io::Read;

// converts a string to a u128
fn str_u128(src: &String, err_msg: &[String; 2]) -> u128 {
	return src
		.parse::<u128>()
		.expect(&format!("{}: \"{}\" {}", err_msg[0], src, err_msg[1]));
}

// splits the string every 3 characters from right to left
pub fn separate_nums(src: &String, ui: &Ui) -> Vec<u128> {
	// vector to hold the numbers
	let mut numv: Vec<u128> = Vec::new();
	let src_len = src.len();
	// temporary string to hold the current number
	let mut num_s = String::new();
	for i in (0..src_len).rev() {
		// put a single digit (actually a char that should contain a digit) in the temporary string
		// put it in the string only if it is not a whitespace
		let cur_ch = src.chars().rev().nth(i).unwrap();
		if !cur_ch.is_whitespace() {
			num_s.push_str(&format!("{}", cur_ch));
			// every 3 characters, push the string to the array and empty it
			// but if a whitespace was found, do nothing
			if i % 3 == 0 {
				numv.push(str_u128(&num_s, &ui.nan_err));
				num_s = String::new();
			}
		}
	}
	return numv;
}

pub fn convert(digits: &Digits, ui: &Ui, numv: Vec<u128>) -> String {
	// write "zero" only if the whole number is actually 0
	if numv.len() == 1 && numv[0] == 0 {
		return format!("{}", digits.ones[0]);
	}
	let mut result = String::new();
	let num_count = numv.len();
	if num_count >= 4 {
		eprintln!("\x1b[0;33m{}\x1b[0m", ui.out_range_err);
	}
	// convert all the digits
	for i in 0..num_count {
		// first digit
		if numv[i] / 100 != 0 {
			result += &format!(
				"{} {} ",
				digits.ones[(numv[i] / 100) as usize],
				digits.hundreds[0]
			);
		}
		// last two digits
		if numv[i] % 100 < 20 && numv[i] % 100 != 0 {
			result += &format!("{} ", digits.ones[(numv[i] % 100) as usize]);
		}
		// if the last two digits weren't in the ones array
		else {
			// second digit
			if (numv[i] / 10) % 10 != 0 {
				result += &format!("{} ", digits.tens[((numv[i] / 10) % 10) as usize]);
			}
			// third digit
			if numv[i] % 10 != 0 {
				result += &format!("{} ", digits.ones[(numv[i] % 10) as usize]);
			}
		}
		// 3 digits weight
		if num_count - i > 1 {
			let weight = num_count - i - 1;
			if weight < 4 {
				result += &format!("{} ", digits.hundreds[weight % 4]);
			} else {
				for _ in 0..(weight / 4) + 1 {
					result += &format!("{} ", digits.hundreds[weight % 4]);
				}
			}
		}
	}
	return result;
}

pub fn read_lang_file(name: String) -> String {
	let mut file = File::open(name).expect("Failed to open file!");
	let mut content = String::new();
	file.read_to_string(&mut content)
		.expect("Failed to read file!");
	return content;
}
