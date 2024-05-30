pub mod tcudp;

fn add(left: usize, right: usize) -> usize {
    tcudp::sock::sock_fn();
    tcudp::tcputil::tcp_fn();
    tcudp::udputil::udp_fn();
    left + right
}


// #[cfg(test)]
mod tests {
    use super::*;
    // #[test]
    pub fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

use crate::tcudp::sock;

mod bse {
    use super::*;
    mod tst {
        use super::*;
        // 公开方法
        pub fn test_fn() {
            println!("test_fn");
            sock::sock_fn(); tests::it_works();
        }
    }
}
