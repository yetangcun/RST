use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write}; /*  */
use std::thread;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref TCP_SOCK: Mutex<TcpServer> = Mutex::new(TcpServer::init_tcp_soc("127.0.0.1:7878"));
}

pub struct TcpServer {
    addr: String,
    tcp_sock: TcpListener
}

impl TcpServer {
    pub fn init_tcp_soc(addr: &str) -> TcpServer { // 初始化Tcp对象
        let tcp_sock = TcpListener::bind(addr).unwrap();
        TcpServer {
            addr: addr.to_string(),
            tcp_sock
        }
    }
    
    // Tcp接收数据
    pub fn start_tcp_server() {
        
        // 启动一个新线程来处理客户端的连接请求
        thread::spawn(move || {
            let tcp_server = TCP_SOCK.lock().unwrap();
            loop {
                if let Ok((mut recv, _addr)) = tcp_server.tcp_sock.accept() {
                    println!("new client connection from {}", _addr);
                }
            }
        });
        
        // 启动一个新线程来处理接收到的数据
        thread::spawn( || {
            let tcp_server = TCP_SOCK.lock().unwrap();
            loop {
                for recv in tcp_server.tcp_sock.incoming() {
                    // 接收数据
                    let mut buf = [0; 512];
                    let mut recv_clt = recv.unwrap();
                    let bts_size = recv_clt.read(&mut buf).unwrap();
                    let msg = String::from_utf8_lossy(&buf[..bts_size]);
                    println!("Recv {} bytes from {}, {}", bts_size, tcp_server.addr, msg);
                    
                    // 发送数据
                    recv_clt.write(msg.as_bytes()).unwrap();
                    recv_clt.flush().unwrap();
                }
            }
        });

    }

    pub fn tcp_send(clt_addr: &str, msg: &str) {
        let mut tcp_sock = TcpStream::connect(clt_addr).unwrap();
        tcp_sock.write(msg.as_bytes()).unwrap();
        tcp_sock.flush().unwrap();
    }
}
