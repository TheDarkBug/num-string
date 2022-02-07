use sscanf::scanf;
use std::fs::File;
use std::io::Read;

#[derive(Clone, Debug)]
pub struct Ui {
	pub nan_err: [String; 2],
	pub insert_num: String,
	pub convert_btn: String,
	pub clear_btn: String,
}

#[derive(Clone, Debug)]
pub struct Digits {
	pub ones: [String; 20],
	pub tens: [String; 10],
	pub hundreds: [String; 4],
}

impl Ui {
	pub fn new(src: &str) -> Self {
		let lines = src.split("\n");
		let mut ne = [
			String::from("nan_err 0 not set!"),
			String::from("nan_err 1 not set!"),
		];
		let mut cvtb = String::from("convert_btn not set!");
		let mut cltb = String::from("clear_btn not set!");
		let mut ins = String::from("insert_num not set!");
		for l in lines {
			// search for the nan_err value
			let ne_parsed = &scanf!(l, "nan_err {} = \"{}\"", usize, String);
			// if it was found, put it in ne
			if !(*ne_parsed).is_err() {
				ne[ne_parsed.as_ref().unwrap().0] = ne_parsed.as_ref().unwrap().1.clone();
			}
			// same as before, but with insert_num
			let ins_parsed = scanf!(l, "insert_num = \"{}\"", String);
			if !ins_parsed.is_err() {
				ins = ins_parsed.unwrap();
			}
			// same as before, but with convert_btn
			let cvtb_parsed = scanf!(l, "convert_btn = \"{}\"", String);
			if !cvtb_parsed.is_err() {
				cvtb = cvtb_parsed.unwrap();
			}
			// same as before, but with clear_btn
			let cltb_parsed = scanf!(l, "clear_btn = \"{}\"", String);
			if !cltb_parsed.is_err() {
				cltb = cltb_parsed.unwrap();
			}
		}
		// generate the new ui struct
		Ui {
			nan_err: ne,
			insert_num: ins,
			convert_btn: cvtb,
			clear_btn: cltb,
		}
	}
}

impl Digits {
	pub fn new(src: &str) -> Self {
		let lines = src.split("\n");
		let mut o: [String; 20] = Default::default();
		let mut t: [String; 10] = Default::default();
		let mut h: [String; 4] = Default::default();
		for l in lines {
			// search for the ones value
			let ones_parsed = scanf!(l, "ones {} = \"{}\"", usize, String);
			// it it was found, put it in the o array at the right position
			if !ones_parsed.is_err() {
				o[ones_parsed.as_ref().unwrap().0] = ones_parsed.as_ref().unwrap().1.clone();
			}
			// same as before, but for the tens
			let tens_parsed = scanf!(l, "tens {} = \"{}\"", usize, String);
			if !tens_parsed.is_err() {
				t[tens_parsed.as_ref().unwrap().0] = tens_parsed.as_ref().unwrap().1.clone();
			}
			// same as before, but for the tens
			let hundreds_parsed = scanf!(l, "hundreds {} = \"{}\"", usize, String);
			if !hundreds_parsed.is_err() {
				h[hundreds_parsed.as_ref().unwrap().0] =
					hundreds_parsed.as_ref().unwrap().1.clone();
			}
		}
		// generate the new digits struct
		Digits {
			ones: o,
			tens: t,
			hundreds: h,
		}
	}
}

pub fn read_lang_file(name: String) -> String {
	let mut file = File::open(name).expect("Failed to open file!");
	let mut content = String::new();
	file.read_to_string(&mut content)
		.expect("Failed to read file!");
	return content;
}
