fn main() {

    let mut str = String::from("Hello world!");

    let lens = fr_wd(&str);
    
    println!("{0}", lens);

    // slice_fn();

    let sr = sl_fn(&str);
    
    println!("{sr}");
    
    str.clear();

    sl_num ()
}

fn fr_wd(s:&String) -> usize {
    let bts = s.as_bytes(); // 转化为字节数组

    for (i,&itm) in bts.iter().enumerate() {  // 循环遍历
        if itm == b' ' {
            return i;
        }
    }

    s.len()
}

fn slice_fn () {
    // let s = String::from("hello world");
    let s = "hello world";

    let s1 = &s[..5];
    let s2 = &s[5..];

    println!("{0}", s1);
    println!("{0}", s2);
}

fn sl_fn (s: &String) ->&str {

    let bts = s.as_bytes();

    for (i, &txt) in bts.iter().enumerate() {
        if txt == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn sl_num () {
    
    let nums = [2,3,4,5,6,7];

    let n1 = &nums[..3];

    assert_eq!(n1, &[2,3,4]);

}