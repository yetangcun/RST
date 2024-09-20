use std::net::TcpListener;
use std::io::{Read,Write};
use std::thread;

pub fn start_tcp_server(addr: &str) {
    let listener = TcpListener::bind(addr).unwrap();
    println!("Server is listenning on {}", addr);
    
    // TcpStream, SocketAddr
    while let Ok((mut recv, addr)) = listener.accept() {
        println!("New connection from {}", addr);
        thread::spawn(move || {
            let mut buf = [0; 512];
            loop {
                let n = match recv.read(&mut buf) {
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => panic!("Error reading from socket: {}", e),
                };
                let msg = String::from_utf8_lossy(&buf[..n]);
                println!("Received {} bytes from {}, {}", n, addr, msg);
            }
        });
    }
}