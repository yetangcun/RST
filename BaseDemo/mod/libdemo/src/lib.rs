mod tcudp;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

mod bse {
    mod tst {
        use super::*;

        // 公开方法
        pub fn test_fn() {
            println!("test_fn");
        }
    }
}
