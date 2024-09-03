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