use sscanf::scanf;
#[cfg(not(target_os = "android"))]
use std::fs::File;
#[cfg(not(target_os = "android"))]
use std::io::Read;

#[derive(Clone, Debug)]
pub struct Ui {
	pub insert_num: String,
	pub convert_btn: String,
	pub clear_btn: String,
	pub copy_btn: String,
}

#[derive(Clone, Debug)]
pub struct Digits {
	pub ones: [String; 20],
	pub tens: [String; 10],
	pub hundreds: [String; 4],
}

#[cfg(not(target_os = "android"))]
impl Ui {
	pub fn new(src: &str) -> Self {
		let lines = src.split("\n");
		let mut cvtb = String::from("convert_btn not set!");
		let mut cltb = String::from("clear_btn not set!");
		let mut cptb = String::from("copy_btn not set!");
		let mut ins = String::from("insert_num not set!");
		for l in lines {
			// search for the insert_num value
			let ins_parsed = scanf!(l, "insert_num = \"{}\"", String);
			// if it was found, put it in ne
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
			// same as before, but with clear_btn
			let cptb_parsed = scanf!(l, "copy_btn = \"{}\"", String);
			if !cptb_parsed.is_err() {
				cptb = cptb_parsed.unwrap();
			}
		}
		// generate the new ui struct
		Ui {
			insert_num: ins,
			convert_btn: cvtb,
			clear_btn: cltb,
			copy_btn: cptb,
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

#[cfg(not(target_os = "android"))]
pub fn read_lang_file(name: String) -> String {
	let mut file = File::open(&name).expect(&format!("Failed to open \"{}\"!", &name));
	let mut content = String::new();
	file.read_to_string(&mut content)
		.expect(&format!("Failed to read \"{}\"!", &name));
	return content;
}
