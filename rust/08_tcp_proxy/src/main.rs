use std::str;
use std::{io::Read, net::TcpListener};

fn sample_listener() {
    let listener = TcpListener::bind("127.0.0.1:8018").unwrap();
    println!("bound the address..");

    let (mut socket, _) = listener.accept().unwrap();
    let mut buffer = Vec::new();
    let n = socket.read_to_end(&mut buffer);
    if let Ok(i) = n {
        println!("{}", i);
        println!("{}", str::from_utf8(&buffer).unwrap());
    }
}

fn main() {
    sample_listener();
}
