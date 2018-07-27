extern crate ethkey;

use self::ethkey::{Brain, BrainPrefix, Generator};

const BRAIN_WORDS: usize = 12;

fn main() {
    // generate
    let mut brain = BrainPrefix::new(vec![0], usize::max_value(), BRAIN_WORDS);
    let keypair = brain.generate().unwrap().to_owned();

    // phrase
    let phrase = brain.phrase();
    println!("Phrase: {:?}", &phrase);
    let validate = validate_phrase(&phrase);
    println!("Validate: {:?}", validate);

    // secret
    let secret = keypair.secret().to_owned();
    println!("Secret: {:?}", secret);

    // public
    let public = keypair.public().to_owned();
    println!("Public: {:?}", public);

    // address
    let address = keypair.address().to_owned();
    println!("Address: {:?}", address);

    // recover from phrase
    let g = Brain::new(phrase.to_string()).generate().unwrap();
    println!("Recover: {:?}", g.address());
}

fn validate_phrase(phrase: &str) -> bool {
    match Brain::validate_phrase(phrase, BRAIN_WORDS) {
	Ok(()) => true,
	Err(_err) => false
    }
}
