use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write}; /*  */
use std::thread;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref TCP_SOCK: Mutex<Tcplib> = Mutex::new(Tcplib::init_tcp_soc("192.168.30.166:7878"));
}

pub struct Tcplib {
    addr: String,
    tcp_sock: TcpListener
}

impl Tcplib {
    pub fn init_tcp_soc(addr: &str) -> Tcplib { // 初始化Tcp对象
        let tcp_sock = match TcpListener::bind(addr) {
            Ok(tcp_lk) => tcp_lk,
            Err(e) => {
                println!("tcp start failed: {}", e);
                panic!("{}", e);
            }
        };
        
        Tcplib { 
            addr: addr.to_string(), 
            tcp_sock
        }
    }
    
    // Tcp接收数据
    pub fn start_tcp_server() {
        
        // 启动一个新线程来处理客户端的连接请求
        let accept_hdl = thread::spawn(move || {
            let tcp_server = match TCP_SOCK.lock() {
                Ok(tcp_obj) => tcp_obj,
                Err(e) => {
                    println!("tcp start failed: {}", e);
                    panic!("{}", e);
                }
            };
            
            println!("tcp server info {}, {}", tcp_server.addr, tcp_server.tcp_sock.local_addr().unwrap());
            
            for recv in tcp_server.tcp_sock.incoming() {
                if let Ok(stream) = recv {
                    
                    println!("tcp client connected {}", stream.peer_addr().unwrap());
                    
                    // 开启新线程接收数据
                    thread::spawn(move || { 
                        Self::tcp_hdl_data(stream)
                    });
                    
                } else {
                    println!("tcp client connect failed");
                }
            }
            
        });
        
        if let Err(_e) = accept_hdl.join() {
            println!("accept thread quit or over!")
        }
    }

    fn tcp_hdl_data (mut stream: TcpStream) {
        // 接收数据
        loop {
            let mut buf = [0; 512];
            let bts_size = stream.read(&mut buf).unwrap();
            let msg = String::from_utf8_lossy(&buf[..bts_size]);
            println!("Recv {} bytes from {}, {}", bts_size, stream.peer_addr().unwrap(), msg);
            
            // 发送数据
            stream.write(msg.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }

    // Tcp发送数据
    pub fn tcp_send(clt_addr: &str, msg: &str) {
        let mut tcp_sock = TcpStream::connect(clt_addr).unwrap();
        tcp_sock.write(msg.as_bytes()).unwrap();
        tcp_sock.flush().unwrap();
    }
}
