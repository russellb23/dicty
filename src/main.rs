// CLI utility to fetch meaning from https://www.thefreedictionary.com
// Developer: Russell B
// Datum: Friday Dec 18th 2020
// bertrussell23@yahoo.in
// Version: 0.1.0
// TODO:
//      1. Receive inputs as arguments
//      [DONE: Dec 18th 2020, 18::56]
//      2. Choose different dictionary sites: Wikictionary, Mariam-Webster etc.,
//      3. Improve text processing to show noun, verb, adjective etc.,
//      [DONE: Dec 19th 2020, 10:07:31]
//      4. Highlight colours on the terminal! (possibly for aesthetics)

extern crate dicty;

//use colored::Colorize;
use dicty::disp_meaning;
use dicty::get_meaning;
use dicty::help;
use std::env;

fn main() {
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
    disp_meaning(meaning);

    //for synon in &meaning.unwrap() {
    //    match synon.as_str() {
    //        "Verb" => {
    //            println!("{}", synon.to_owned().italic().bold().yellow());
    //        }
    //        "Transitive" => {
    //            println!("{}", synon.to_owned().italic().bold().magenta());
    //        }
    //        "Intransitive" => {
    //            println!("{}", synon.to_owned().italic().bold().cyan());
    //        }
    //        "Noun" => {
    //            println!("{}", synon.to_owned().italic().bold().blue());
    //        }
    //        "Adjective" => {
    //            println!("{}", synon.to_owned().italic().bold().green());
    //        }
    //        "Adverb" => {
    //            println!("{}", synon.to_owned().italic().bold().purple());
    //        }
    //        "Intransitive Verb" => {
    //            println!("{}", synon.to_owned().italic().bold().red());
    //        }
    //        "Transitive Verb" => {
    //            println!("{}", synon.to_owned().italic().bold().red());
    //        }
    //        _ => {
    //            println!("{}\n", synon);
    //        }
    //    }
    //}
}
