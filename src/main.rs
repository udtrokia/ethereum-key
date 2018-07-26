extern crate ethkey_cli;

use std::{env, process};

use ethkey_cli::cli::{env_logger, execute, panic_hook, Error};


fn main() {
    panic_hook::set_abort();
    env_logger::init().expect("Logger initialized only once.");
    
    match execute(env::args()) {
 	Ok(ok) => println!("{}", ok),
 	Err(Error::Docopt(ref e)) => e.exit(),
 	Err(err) => {
 	    println!("{}", err);
 	    process::exit(1);
 	}
    }
}
