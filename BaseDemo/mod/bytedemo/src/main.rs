use libdemo::tcudp::*;
use bytedemo::extensions::*;

fn main() {
    println!("Hello, world!");
    sock::sock_fn();
    tcputil::tcp_fn();
    udputil::udp_fn();

    mysqlutil::getconn();
    discacheutil::setcache("ky001", "vl007");
}
