use clap::Parser;
use std::fs::File;
use std::io::{stdin, stdout, Write};
mod conversion;
use conversion::*;
mod interface;
use interface::*;
#[cfg(not(target_os = "android"))]
mod graphics;

#[derive(Parser, Debug)]
#[clap(author = "Adriano Oliviero (TheDarkBug)", version = "2.1", about = "A program that write numbers as they are pronounced", long_about = None)]
struct Args {
    /// Disable gtk gui.
    #[clap(short = 'g', long)]
    nogui: bool,

    /// Change language.
    #[clap(short, long, default_value_t = String::from("langs/english.txt"))]
    lang: String,

    /// Set output file.
    #[clap(short, long, default_value_t = String::from("stdout"))]
    out: String,
}

fn main() {
    #[cfg(not(target_os = "android"))]
    let args = Args::parse();
    #[cfg(target_os = "android")]
    let mut args = Args::parse();

    // loading the language
    let lang = read_lang_file(args.lang);
    let ui = Ui::new(&lang);
    let digits = Digits::new(&lang);

    // disable gui on android
    #[cfg(target_os = "android")]
    {
        args.nogui = true;
    }

    // run tui version only if gui disabled
    if args.nogui {
        // get user input
        print!("{} ", ui.insert_num);
        stdout().flush().expect("");
        let mut user_input = String::new();
        stdin().read_line(&mut user_input).expect("");
        // remove the '\n' from the input
        user_input.remove(user_input.len() - 1);
        if args.out == "stdout" || args.out == "/dev/stdout" {
            println!("{}", convert(&digits, separate_nums(&mut user_input)));
        } else {
            writeln!(
                File::create(&args.out).expect(&format!("Failed to open {}: {0}", &args.out)),
                "{}",
                convert(&digits, separate_nums(&mut user_input))
            )
            .expect(&format!("Failed to write to {}!", &args.out));
            return;
        }
    } else {
        #[cfg(not(target_os = "android"))]
        graphics::run(ui.clone(), digits.clone());
    }
}
