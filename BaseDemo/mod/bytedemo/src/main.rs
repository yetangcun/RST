use libdemo::tcudp::*;

fn main() {
    println!("Hello, world!");
    sock::sock_fn();
    tcputil::tcp_fn();
    udputil::udp_fn();
}
