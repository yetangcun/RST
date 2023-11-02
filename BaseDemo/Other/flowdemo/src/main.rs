fn main() {
    let temp = "xxx";
    let condi = true;
    let res = if condi {temp} else {"zzz"};
    println!("the res is {res}");

    // if条件控制
    if_fn();

    // loop循环
    loop_fn();

    // while循环
    while_fn();

    // for循环
    for_fn();
}


fn if_fn() {
    let varial = 111;

    // if条件控制
    if varial%2==0 {
        println!("the result is sure");
    } else if varial>110 {
        println!("the result is most big");
    } else {
        println!("the result is not sure");
    }
}


fn loop_fn() {
    let mut counts = 11; // mut修饰 可变变量
    let last_val = loop {
        if counts<1 {
            break "counts is over";
        }
        counts-=1;
        println!("Hi, good luck!");
    };
    println!("attentions, {last_val}!");
}



fn while_fn() {
    let mut counts = 0; // mut修饰 可变变量
    while counts<12 {
        println!("now, curr counts val is {counts}!");
        counts+=1;
    }
    println!("attentions, last counts val is {counts}!");
}



fn for_fn() {
    let counts = 12; // mut修饰 可变变量
    let count=counts + 1;
    for num in 1..count {
        println!("again sure, curr num is {num}");
    }
    println!("well, all is over now!!!");
}
