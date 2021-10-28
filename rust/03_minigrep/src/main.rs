// See: An I/O project at https://doc.rust-lang.org/book/ch12-00-an-io-project.html
extern crate minigrep;

use std::env;
use std::error::Error;
use std::process;
use minigrep::Config;

fn main() -> Result<(), Box<dyn Error>>{
    let args: Vec<String> = env::args().collect();

    // // let config = Config::new(&args);
    // // match can be refactored into unwrap_or_else
    // match config {
    //     Ok(c) => println!("{:?}", c),
    //     Err(msg) => println!("{}", msg),        
    // };

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Porblem parsing arguments: {}", err);
        process::exit(1);
    });

    minigrep::run(config)
}
