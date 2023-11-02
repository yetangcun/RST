fn main() {
    let varial = 111;

    // if条件控制
    if varial%2==0 {
        println!("the result is sure");
    } else if varial>110 {
        println!("the result is most big");
    } else {
        println!("the result is not sure");
    }

    let temp = "xxx";
    let condi = true;
    let res = if condi {temp} else {"zzz"};
    println!("the res is {res}");

    // loop循环
    let mut counts = 11; // mut修饰 可变变量
    let last_val = loop {
        if counts<1 {
            break "counts is over";
        }
        counts-=1;
        println!("Hi, good luck!");
    };
    println!("attentions, {last_val}!");

    // while循环
    while counts<12 {
        println!("now, curr counts val is {counts}!");
        counts+=1;
    }
    println!("attentions, last counts val is {counts}!");

    // for循环
    let count = counts + 1;
    for num in (1..count).rev() {
        println!("again sure, curr num is {num}");
    }
    println!("well, all is over now!!!");
}
