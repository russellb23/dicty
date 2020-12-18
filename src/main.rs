// CLI utility to fetch meaning from https://www.thefreedictionary.com
// Developer: Russell B
// Datum: Friday Dec 18th 2020
// bertrussell23@yahoo.in
// Version: 0.1.0
// TODO:
//      1. Receive inputs as arguments [DONE: Dec 18th 2020, 18::56]
//      2. Choose different dictionary sites: Wikictionary, Mariam-Webster etc.,
//      3. Improve text processing to show noun, verb, adjective etc.,
//      4. Highlight colours on the terminal! (possibly for aesthetics)

//extern crate clap;
extern crate dicsyn;

//use clap::{App, Arg};

//use argh::FromArgs;
use dicsyn::get_meaning;
use dicsyn::help;
use std::env;

//#[derive(FromArgs)]
///// Command Line arguments parsing
//struct Opts {
//    /// get meaning for this word
//    #[argh(short = 'w', option)]
//    word: String,
//
//    /// get this many meanings, if available
//    #[argh(short = 'n', option)]
//    n: usize,
//}

fn main() {
    //println!("Welcome to the dictionary CLI!");

    let args: Vec<String> = env::args().collect();
    let mut kword: String = String::new();
    // Setting default 2
    let mut num: usize = 2;

    match args.len() {
        // no arguments passed
        1 => {
            println!("Dictionary app needs a word to find meaning!")
        }
        2 => {
            // Extract keywords
            kword = match args[1].trim().parse() {
                Ok(word) => word,
                Err(_) => panic!("Check the input keyword"),
            };
        }
        3 => {
            // Extract keyword
            kword = match args[1].trim().parse() {
                Ok(word) => word,
                Err(_) => panic!("Check the input keyword"),
            };
            // Extract number of meanings
            num = match args[2].trim().parse::<usize>() {
                Ok(n) => n,
                Err(_) => panic!("Invalid number of entries requested"),
            };
        }
        _ => {
            // show help
            help();
        }
    }

    let meaning = get_meaning(&kword, num.clone());

    println!("Meaning(s):\n");
    for synon in &meaning.unwrap() {
        println!("{:?}\n", synon);
    }
}
