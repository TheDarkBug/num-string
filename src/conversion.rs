use crate::interface::*;

// converts a string to a u128
fn str_u128(src: &String) -> u128 {
	return src.parse::<u128>().expect("Unreachable");
}

// splits the string every 3 characters from right to left
pub fn separate_nums(src: &str) -> Vec<u128> {
	// remove non-number characters
	let dsrc = src.replace(|c| c > '9' || c < '0', "");
	// vector to hold the numbers
	let mut numv: Vec<u128> = Vec::new();
	let src_len = dsrc.len();
	// temporary string to hold the current number
	let mut num_s = String::new();
	for i in (0..src_len).rev() {
		// put a single digit (actually a char that should contain a digit) in the temporary string
		// put it in the string only if it is not a whitespace
		let cur_ch = dsrc.chars().rev().nth(i).unwrap();
		if !cur_ch.is_whitespace() {
			num_s.push_str(&format!("{}", cur_ch));
			// every 3 characters, push the string to the array and empty it
			// but if a whitespace was found, do nothing
			if i % 3 == 0 {
				numv.push(str_u128(&num_s));
				num_s = String::new();
			}
		}
	}
	return numv;
}

pub fn convert(digits: &Digits, numv: Vec<u128>) -> String {
	// write "zero" only if the whole number is actually 0
	if numv.len() == 1 && numv[0] == 0 {
		return format!("{}", digits.ones[0]);
	}
	let mut result = String::new();
	let num_count = numv.len();
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
