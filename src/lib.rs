extern crate reqwest;
extern crate select;

use colored::Colorize;
use http::StatusCode;
use reqwest::Result;
use select::document::Document;
use select::predicate::Class;

#[tokio::main]
pub async fn get_meaning(s: &str, n: usize) -> Result<Vec<String>> {
    let mut freedict_url: String = "https://www.thefreedictionary.com/".to_owned();
    let mut meaning: Vec<String> = Vec::new();
    // Append the word to the URL string
    freedict_url.push_str(s);
    //Get the response from the provided URL
    let resp = reqwest::get(&freedict_url).await?;

    // Match the response from the server
    // This has to be generalized as much as possible
    // accounting the range of response codes
    match resp.status() {
        StatusCode::OK => {
            // Evaluate the response
            //assert!(resp.status().is_success());

            // Extract text in an asyncronous manner
            let body = resp.text().await.unwrap();

            // Convert the String to Document
            let document = Document::from(body.as_str());

            // Find the class correspond to the meaning
            // pseg class has all the meanings
            // It's children are the meanings for the word
            for node in document.find(Class("pseg")).take(n) {
                let mm = node.children();
                for meang in mm {
                    if meang.text() == "n." {
                        meaning.push("Noun".to_string());
                    } else if meang.text() == "v." {
                        meaning.push("Verb".to_string());
                    } else if meang.text() == "tr." {
                        meaning.push("Transitive".to_string());
                    } else if meang.text() == "adj." {
                        meaning.push("Adjective".to_string());
                    } else if meang.text() == "adv." {
                        meaning.push("Adverb".to_string());
                    } else if meang.text() == " " {
                        meaning = meaning;
                    } else if meang.text() == "intr." {
                        meaning.push("Intransitive".to_string());
                    } else if meang.text() == ", " {
                        meaning = meaning;
                    } else if meang.text() == "tr.v." {
                        meaning.push("Transitive Verb".to_string());
                    } else {
                        meaning.push(meang.text());
                    }
                }
            }
        }
        _ => panic!("Correct spelling or grammar mistakes, if any"),
    }

    // Return the Vector of Meaning Strings
    Ok(meaning)
}

pub fn help() {
    println!("Usage:\ndicsyn <key_word> <num_of_meanings>(optional)");
}

/// Display the meaning(s) of the queried word as a Vec<String> encapsultaed in Result
pub fn disp_meaning(meaning: Result<Vec<String>>) {
    let meanings: Vec<String> = match meaning {
        Ok(vec_val) => vec_val,
        Err(_) => panic!("Verify the keyword for any spelling or grammar mistakes"),
    };

    for synon in meanings {
        match synon.as_str() {
            "Verb" => {
                println!("{}", synon.to_owned().italic().bold().yellow());
            }
            "Transitive" => {
                println!("{}", synon.to_owned().italic().bold().magenta());
            }
            "Intransitive" => {
                println!("{}", synon.to_owned().italic().bold().cyan());
            }
            "Noun" => {
                println!("{}", synon.to_owned().italic().bold().blue());
            }
            "Adjective" => {
                println!("{}", synon.to_owned().italic().bold().green());
            }
            "Adverb" => {
                println!("{}", synon.to_owned().italic().bold().purple());
            }
            "Intransitive Verb" => {
                println!("{}", synon.to_owned().italic().bold().red());
            }
            "Transitive Verb" => {
                println!("{}", synon.to_owned().italic().bold().red());
            }
            _ => {
                println!("{}\n", synon);
            }
        }
    }
}
