use std::net::UdpSocket;
use std::io::{Read,Write};
use std::thread;

pub fn start_udp_server(addr: &str) {
    let sock = UdpSocket::bind(addr).unwrap();
    println!("Server is listenning on {}", addr);
    
    let mut buf = [0; 512];
    // SocketAddr, SocketAddr
    while let (nums, addr) = sock.recv_from(&mut buf).unwrap() {
        println!("New connection from {}", addr);
        thread::spawn(move || {
            let msg = String::from_utf8_lossy(&buf[..nums]);
            println!("Received {} bytes from {}, {}", nums, addr, msg);
        });
    }
}

pub fn udp_server(addr: &str) {
    let sock = UdpSocket::bind(addr).unwrap();
    println!("Server is listenning on {}", addr);
    
    loop {
        let mut buf = [0; 512];
        let (nums, addr) = sock.recv_from(&mut buf).unwrap();
        let msg = String::from_utf8_lossy(&buf[..nums]);
        println!("Received {} bytes from {}, {}", nums, addr, msg);
        //sock.send_to(msg.as_bytes(), addr).unwrap();
        sock.send_to(msg.as_bytes(), "127.0.0.1:7878").unwrap();
    }
}