#[macro_use]
extern crate log;

use std::error::Error;
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream, ToSocketAddrs};
use std::{str, thread};

// const lives for the entire lifetime of the program
const SOCKS_VERSION: u8 = 0x05;
const SUPPORTED_METHOD: u8 = 0x02;
const NO_SUPPORTED_METHOD: u8 = 0xff;
const CREDENTIAL_ERROR: u8 = 0xff;
const CREDENTIAL_VERSION: u8 = 0x01;
const U: &str = "username";
const P: &str = "password";

fn get_methods(stream: &mut TcpStream, n: u8) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut methods = Vec::new();

    for _ in 0..n {
        let mut buf = [0; 1];
        stream.read_exact(&mut buf)?;
        methods.push(buf[0]);
    }
    Ok(methods)
}

fn check_credentials(stream: &mut TcpStream) -> bool {
    let mut credentials_buf = vec![0; 256];
    let n = stream.read(&mut credentials_buf).unwrap();

    println!("Credential length: {}", n);

    // version
    let version = credentials_buf[0];
    assert!(version == CREDENTIAL_VERSION);

    // username
    let ulen = usize::from(credentials_buf[1]);
    let username = str::from_utf8(&credentials_buf[2..ulen + 2]).unwrap();
    println!("Username: {:#?}", username);

    // password
    let plen = usize::from(credentials_buf[ulen + 2]);
    let (pstart, pend) = (ulen + 2 + 1, ulen + 2 + 1 + plen);
    let password = str::from_utf8(&credentials_buf[pstart..pend]).unwrap();
    println!("Password: {:#?}", password);

    if username == U && password == P {
        println!("=================");
        stream.write(&mut vec![version, 0x00]).unwrap();
        true
    } else {
        stream.write(&mut vec![version, CREDENTIAL_ERROR]).unwrap();
        false
    }
}

fn exchange_loop(source: &mut TcpStream, target: &mut TcpStream) {
    // Since TCP streams are bidirectional, so we use try_clone and
    // two threads to pipe the data exchange of client <-> proxy <-> target host
    // at the same time
    let mut source_read = source.try_clone().unwrap();
    let mut source_write = source.try_clone().unwrap();

    let mut target_read = target.try_clone().unwrap();
    let mut target_write: TcpStream = target.try_clone().unwrap();

    let mut handles = vec![];
    let h1 = thread::spawn(move || loop {
        let mut buf = [0; 4096];
        let n = source_read.read(&mut buf).unwrap();
        println!("Source read: {}", n);
        if n <= 0 {
            break;
        }

        let n = target_write.write(&buf[..n]).unwrap();
        if n <= 0 {
            break;
        }
    });
    handles.push(h1);

    let h2 = thread::spawn(move || loop {
        let mut buf = [0; 4096];
        let n = target_read.read(&mut buf).unwrap();
        println!("Target read: {}", n);
        if n <= 0 {
            break;
        }

        let n = source_write.write(&buf[..n]).unwrap();
        if n <= 0 {
            break;
        }
    });
    handles.push(h2);

    for handle in handles {
        handle.join().unwrap();
    }
}

fn main() {
    // Sample: curl -v --socks5 127.0.0.1:8081 -U username:password https://github.com
    let listener = TcpListener::bind("127.0.0.1:8081").unwrap();
    let (mut socket, _) = listener.accept().unwrap();

    // Reader the first 2 bytes for greating
    let mut buf = [0; 2];
    socket.read_exact(&mut buf).unwrap();

    let [version, n_methods] = buf;
    println!("Version: {}", version);
    println!("Num methods: {}", n_methods);

    // Make sure the corrected socks version is supported
    assert!(version == SOCKS_VERSION);
    assert!(n_methods > 0);

    // Get available methods
    let methods = get_methods(&mut socket, n_methods).unwrap();
    for method in &methods {
        println!("{}", method);
    }

    if !methods.contains(&SUPPORTED_METHOD) {
        socket
            .write(&mut vec![SOCKS_VERSION, NO_SUPPORTED_METHOD])
            .unwrap();
        return;
    }
    // Send welcome message
    socket
        .write(&mut vec![SOCKS_VERSION, SUPPORTED_METHOD])
        .unwrap();

    // Verify
    if !check_credentials(&mut socket) {
        panic!("Incorrect USERNAME/PASSWORD!")
    }

    // Get client request
    let mut req_buf = [0; 4];
    socket.read_exact(&mut req_buf).unwrap();
    let [version, cmd, _, address_type] = req_buf;

    let socket_addr = if address_type == 1 {
        let mut addr_buf = [0; 4];
        socket.read_exact(&mut addr_buf).unwrap();
        println!("Target address: {:?}", addr_buf);

        let mut port_buf = [0; 2];
        socket.read_exact(&mut port_buf).unwrap();
        let port = u16::from_be_bytes(port_buf);
        println!("Port: {:?}", port);

        let [v1, v2, v3, v4] = addr_buf;
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(v1, v2, v3, v4)), port)
    } else {
        let mut domain_len_buf = [0; 1];
        socket.read_exact(&mut domain_len_buf).unwrap();

        let domain_len = usize::from(domain_len_buf[0]);
        let mut domain_buf = [0; 256];
        socket.read_exact(&mut domain_buf).unwrap();

        let domain = str::from_utf8(&domain_buf[0..domain_len]).unwrap();
        domain.to_socket_addrs().unwrap().next().unwrap()
    };

    // Try to connect target host
    if cmd == 1 {
        let mut target = TcpStream::connect(socket_addr).unwrap();
        let target_addr = target.peer_addr().unwrap();
        println!("Target {:?} connected.", target_addr);

        // Reply to client that the target is connected
        println!("Target IP: {:?}", target_addr.ip());
        let ip = if let IpAddr::V4(ip) = target_addr.ip() {
            ip.octets().to_vec()
        } else {
            todo!()
        };

        println!("{:?}", ip);

        // Notify the client that the target host has been connected
        let mut connected = vec![SOCKS_VERSION, 0, 0, address_type];
        connected.extend(ip.iter());
        connected.extend(target_addr.port().to_be_bytes());
        socket.write(&connected[..]).unwrap();

        // Establish data exchange
        exchange_loop(&mut socket, &mut target)
    } else {
        todo!();
    }

    println!("{:?}", socket_addr);
}
