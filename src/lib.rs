extern crate reqwest;
extern crate select;

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

    // Evaluate the response
    assert!(resp.status().is_success());

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
            meaning.push(meang.text());
        }
    }

    // Return the Vector of Meaning Strings
    Ok(meaning)
}

pub fn help() {
    println!("Usage:\ndicsyn <key_word> <num_of_meanings>(optional)");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
