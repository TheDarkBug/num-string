use getopts::Options;
use std::fs::File;
use std::io::{stdin, stdout, Write};
mod conversion;
use conversion::*;
mod interface;
use interface::*;
#[cfg(not(target_os = "android"))]
mod graphics;

fn usage(program: &str, opts: Options) {
    print!("{}", opts.usage(&format!("Usage: {} [options]", program)));
}

fn main() {
    // get command line options
    let argv: Vec<String> = std::env::args().collect();
    let mut opts = Options::new();
    opts.optflag("h", "help", "prints this help message");
    opts.optopt("l", "lang", "set language file", "FILE");
    opts.optopt("o", "out", "set output file", "FILE");
    opts.optflag("g", "no-graphics", "use only terminal interface");
    let matches = match opts.parse(&argv[1..]) {
        Ok(m) => m,
        Err(f) => panic!("{}", f.to_string()),
    };
    if matches.opt_present("help") {
        usage(&argv[0], opts);
        return;
    }

    let graphics_enabled: bool;
    #[cfg(target_os = "android")]
    {
        // disable graphics on android
        graphics_enabled = false;
    }
    #[cfg(not(target_os = "android"))]
    if matches.opt_present("no-graphics") {
        graphics_enabled = false;
    } else {
        graphics_enabled = true;
    }
    // run tui version only if gui disabled
    if !graphics_enabled {
        // get language file name option (or default)
        let lang: String;
        let lang_file_name: String;
        if matches.opt_present("lang") {
            match matches.opt_str("l") {
                Some(x) => lang_file_name = String::from(x.as_str()),
                None => {
                    eprintln!("Error: language file was not provided!");
                    std::process::exit(1); // exit with error
                }
            }
        } else {
            // default language file name
            lang_file_name = String::from("langs/english.txt");
        }
        lang = read_lang_file(lang_file_name);
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
        if matches.opt_present("out") {
            match matches.opt_str("o") {
                Some(x) => {
                    if !(x == "stdout" || x == "/dev/stdout") {
                        writeln!(
                            File::create(&x).expect(&format!("Failed to open {}: {0}", &x)),
                            "{}",
                            convert(&digits, &ui, separate_nums(&user_input, &ui))
                        )
                        .expect(&format!("Failed to write to {}!", &x));
                        return;
                    }
                }

                None => {}
            }
        } else {
            println!("{}", convert(&digits, &ui, separate_nums(&user_input, &ui)));
        }
    } else {
        #[cfg(not(target_os = "android"))]
        {
            // get language file name option (or default)
            let lang: String;
            let lang_file_name: String;
            if matches.opt_present("lang") {
                match matches.opt_str("l") {
                    Some(x) => lang_file_name = String::from(x.as_str()),
                    None => {
                        eprintln!("Error: language file was not provided!");
                        std::process::exit(1); // exit with error
                    }
                }
            } else {
                // default language file name
                lang_file_name = String::from("langs/english.txt");
            }
            lang = read_lang_file(lang_file_name);
            // load the language
            let ui = Ui::new(&lang);
            let digits = Digits::new(&lang);
            graphics::run(ui.clone(), digits.clone());
        }
    }
}
