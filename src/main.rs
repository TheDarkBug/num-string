use std::fs::File;
use std::io::{stdin, stdout, Read, Write};
extern crate sscanf;

struct Digits {
    ones: [String; 20],
    tens: [String; 10],
    hundreds: [String; 4],
}

struct Ui {
    out_range_err: String,
    nan_err: [String; 2],
    insert_num: String,
}

impl Ui {
    fn new(src: &str) -> Self {
        let lines = src.split("\n");
        let mut ore = String::from("out_range_err not set!");
        let mut ne = [
            String::from("nan_err 0 not set!"),
            String::from("nan_err 1 not set!"),
        ];
        let mut ins = String::from("insert_num not set!");
        for l in lines {
            // search for the out_range_err value
            let ore_parsed = sscanf::scanf!(l, "out_range_err = \"{}\"", String);
            // if it was found, put it in ore
            if ore_parsed != None {
                ore = ore_parsed.unwrap();
            }
            // same as before, but with nan_err
            let ne_parsed = &sscanf::scanf!(l, "nan_err {} = \"{}\"", usize, String);
            if *ne_parsed != None {
                ne[ne_parsed.as_ref().unwrap().0] = ne_parsed.as_ref().unwrap().1.clone();
            }
            // same as before, but with insert_num
            let ins_parsed = sscanf::scanf!(l, "insert_num = \"{}\"", String);
            if ins_parsed != None {
                ins = ins_parsed.unwrap();
            }
        }
        // generate the new ui struct
        Ui {
            out_range_err: ore,
            nan_err: ne,
            insert_num: ins,
        }
    }
}

impl Digits {
    fn new(src: &str) -> Self {
        let lines = src.split("\n");
        let mut o: [String; 20] = Default::default();
        let mut t: [String; 10] = Default::default();
        let mut h: [String; 4] = Default::default();
        for l in lines {
            // search for the ones value
            let ones_parsed = sscanf::scanf!(l, "ones {} = \"{}\"", usize, String);
            // it it was found, put it in the o array at the right position
            if ones_parsed != None {
                o[ones_parsed.as_ref().unwrap().0] = ones_parsed.as_ref().unwrap().1.clone();
            }
            // same as before, but for the tens
            let tens_parsed = sscanf::scanf!(l, "tens {} = \"{}\"", usize, String);
            if tens_parsed != None {
                t[tens_parsed.as_ref().unwrap().0] = tens_parsed.as_ref().unwrap().1.clone();
            }
            // same as before, but for the tens
            let hundreds_parsed = sscanf::scanf!(l, "hundreds {} = \"{}\"", usize, String);
            if hundreds_parsed != None {
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

// converts a string to a u128
fn str_u128(src: &String, err_msg: &[String; 2]) -> u128 {
    return src.parse::<u128>().expect(&format!(
        "{}: \"{}\" {}",
        err_msg[0],
        src.chars().rev().collect::<String>(),
        err_msg[1]
    ));
}

// splits the string every 3 characters from right to left
fn separate_nums(src: &String, ui: &Ui) -> Vec<u128> {
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

fn convert(digits: Digits, ui: &Ui, numv: Vec<u128>) -> String {
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
                println!("W: {}", weight / 3);
                for j in 0..(weight / 4) + 1 {
                    result += &format!("{} ", digits.hundreds[weight % 4]);
                }
            }
        }
    }
    return result;
}

fn read_lang_file(name: String) -> String {
    let mut file = File::open(name).expect("Failed to open file!");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed to read file!");
    return content;
}

fn main() {
    // get eventual language file name
    let argv: Vec<String> = std::env::args().collect();
    let lang: String;
    if argv.len() < 2 {
        // default language
        lang = read_lang_file(String::from("langs/english.txt"));
    } else {
        // argument language
        lang = read_lang_file(argv[1].clone());
    }
    // load the language
    let ui = Ui::new(&lang);
    let digits = Digits::new(&lang);
    // get user input
    print!("{} ", ui.insert_num);
    stdout().flush().expect("");
    let mut user_input = String::new();
    stdin().read_line(&mut user_input).expect("");
    // remove the '\n' from the input
    user_input.remove(user_input.len() - 1);
    println!("{}", convert(digits, &ui, separate_nums(&user_input, &ui)));
}
