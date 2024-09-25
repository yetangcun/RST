use std::io;
use TcudpLib::tcudp::{rstcp::Tcplib, rsudp::Udplib};

fn main() {
    
    // tcp_test();

    udp_test();
    
}

fn tcp_test() {
    Tcplib::start_tcp_server();
}

fn udp_test() {
    Udplib::udp_server();
}
