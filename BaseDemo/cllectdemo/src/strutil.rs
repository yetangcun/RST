pub fn str_fn() {
    // rust字符串不支持索引取值
    let mut str = String::new();
    str.push_str("hello,");

    let mut str0 = "欢迎您".to_string();  // 等价于String::from("欢迎您")
    let str1 = String::from("my friends");
    let str2 = str + &str1;  // str被移动到str2中, str失效，str1继续有效可用

    println!("str0 is: {str0}, str1 is: {str1}, str2 is: {str2}");

    str0.push_str(",最亲密的朋友"); // push_str方法追加字符串
    str0.push('!'); // push方法是追加字符
    println!("Latest str0 is: {str0}");

    // format!宏 使用的都是字符串的引用,不获取所有权
    let s1 = String::from("66");
    let s2 = String::from("99");
    let s3 = String::from("88");
    let s = format!("{s1}-{s2}-{s3}");
    println!("s1:{s1},s2:{s2},s3:{s3},s:{s}");
    let si = &s1[0..1]; println!("si:{si}");

    // 一个汉字占三个字节
    let helo = "Здравствуйте"; // 人间疾苦
    let lens = helo.len();
    let ss = &helo[0..4];
    println!("ss:{ss}, {lens}");

    for hl in helo.chars() {
        println!("hl:{hl}");
    }
    
    for hl in helo.bytes() {
        println!("bty:{hl}");
    }
    
    // find函数 间
    let has = helo.find("т").unwrap();
    println!("has:{has}");

    // replace函数
    let ss2 = "hello world";
    let ss3 = ss2.replace("world", "rust");
    println!("ss2:{ss2},ss3:{ss3}");

    // contains函数
    let ss4 = ss3.contains("rust");
    println!("ss4:{ss4}");
}