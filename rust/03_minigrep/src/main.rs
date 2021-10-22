// See: An I/O project at https://doc.rust-lang.org/book/ch12-00-an-io-project.html
extern crate minigrep;

use std::env;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    let config = Config::new(query, filename);
    println!("{:?}", args);
}
