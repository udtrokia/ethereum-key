// ethereum-key; test;
extern crate ethereum_key;
use ethereum_key::source::Band;

fn main() {
    println!("HELLO");
    let band = Band::generate();
    let test = Band::from(band.phrase.to_string());
    println!("{:?}", band);
    println!("{:?}", test);
}
