use std::net::UdpSocket;
// use std::io::{Read, Write};
use std::thread;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref UDP_SOCK: Mutex<Udplib> = Mutex::new(Udplib::init_udp_soc("127.0.0.1:7879"));
}

pub struct Udplib {
    addr: String,
    sock: UdpSocket
}

impl Udplib {
    pub fn init_udp_soc(addr: &str) -> Udplib { // 初始化Udp对象
        
        println!("udp start init on {}", addr);

        let sock_obj = UdpSocket::bind(addr);
        let sock = match sock_obj {
            Ok(udp_sock) => udp_sock,
            Err(e) => {
                println!("udp start failed: {}", e);
                panic!("{}", e);
            }
        };

        println!("udp start bind on {}", sock.local_addr().unwrap());
        
        Udplib {
            addr: addr.to_string(),
            sock: sock
        }
    }
    
    // Udp接收数据
    pub fn start_udp_server() { // 开启一个新线程来处理接收到的数据
        thread::spawn(move || {
            let udp_obj = UDP_SOCK.lock().unwrap();
            println!("Udp Server is listenning on {}", udp_obj.addr);
            
            loop {
                let mut buf = [0; 512];
                let (nums, _addr) = udp_obj.sock.recv_from(&mut buf).unwrap();
                let msg = String::from_utf8_lossy(&buf[..nums]);
                println!("Udp Recv {} bytes from {}, {}", nums, _addr, msg);
            }
        });
    }

    // Udp接收数据
    pub fn udp_server() {
        thread::spawn(move || {
            let udp_obj = UDP_SOCK.lock().unwrap();
            println!("Udp Server is listenning on {}", udp_obj.addr);
            loop {
                let mut buf = [0; 512];
                let (nums, _addr) = udp_obj.sock.recv_from(&mut buf).unwrap();
                let msg = String::from_utf8_lossy(&buf[..nums]);
                println!("Udp Recv {} bytes from {}, {}", nums, _addr, msg);
                // udp_obj.sock.send_to(msg.as_bytes(), "127.0.0.1:7878").unwrap();
            }
        });
    }

    // Udp发送数据
    pub fn udp_clt_send(clt_addr: &str, msg: &str) {
        let udp_obj = UDP_SOCK.lock().unwrap();
        udp_obj.sock.send_to(msg.as_bytes(), clt_addr).unwrap();
    }
}

