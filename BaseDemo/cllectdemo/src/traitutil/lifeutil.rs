pub fn lft_fn() {
    let y = 4;
    let x = &y;
    println!("x:{x}");
}

pub fn lft_fn1() {
    let sr0 = String::from("hello");
    let sr1 = "world!";
    let res = lgest_fn(sr0.as_str(), sr1);
    println!("the lgest str:{res}");
}

pub fn lgest_fn<'a>(sr1:&'a str, sr2:&'a str) -> &'a str {
    if sr1.len() > sr2.len() {
        sr1
    } else {
        sr2
    }
}

pub fn lgest_fn2<'a>(sr1:&str, sr2:&str) -> String {
    let sr = String::from(format!("{}{}", sr1, sr2));
    sr
    // sr.as_str()
}

pub fn lft_fn2() {
    let s = String::from("hello");
    {
        let r1 = String::from("haha");
        // let r1 = "haha";
        let res = lgest_fn(s.as_str(), r1.as_str());
        println!("res1:{res}");
    }
}

pub fn lft_fn3() {
    let s = String::from("hello");
    let res;
    {
        // let r1 = String::from("heihei");
        let r1 = "heihei";
        res = lgest_fn(s.as_str(), r1);
    }
    println!("res2:{res}");
}