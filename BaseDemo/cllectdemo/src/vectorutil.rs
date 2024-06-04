pub fn vec_fn() {
    let v:Vec<i32> = Vec::new();
    let mut vele = vec![1,2,3];  // vec! 宏
    vele.push(4);

    let thrd: &i32 = &vele[0];
    let secd: Option<&i32> = vele.get(1);
    println!("the first vec element is: {thrd}");
    match secd {
        Some(x) => println!("the second vec element is: {x}"),
        None => println!("the second vec element is: None"),
    }

    let mut vec2 = Vec::new();
    vec2.push(1);
    vec2.push(2);
    vec2.push(3);
    vec2.push(4);
    vec2.push(5);
    
    let ele = &vec2[1];
    let vec_len = vec2.len();
    //let lt_ele = &vec2[100];
    let lst_ele = vec2.get(100);
    match lst_ele {
        Some(el) => println!("the last element is: {el}-{ele}"),
        None => println!("the last element is: None, element len is: {vec_len}-{ele}"),
    }

    for i in &mut vec2 {
        *i += 66;
        println!("the vec ele is: {0}",&i);
    }

    // 通过枚举让Vector能间接存储不同类型的数据
    let mut rw = vec![  // 创建一个可变的Vector
        SpdSheetCell::Int(1),
        SpdSheetCell::Float(2.0),
        SpdSheetCell::Text(String::from("hello")),
    ];
    for i in &rw {
        match i {
            SpdSheetCell::Int(x) => println!("the int is: {x}"),
            SpdSheetCell::Float(x) => println!("the float is: {x}"),
            SpdSheetCell::Text(x) => println!("the text is: {x}"),
        }
    }

    let rp:Option<SpdSheetCell> = rw.pop(); // 弹出最后一个加入的元素
    match rp {
        Some(SpdSheetCell::Int(x)) => println!("the int is: {x}"),
        Some(SpdSheetCell::Float(x)) => println!("the float is: {x}"),
        Some(SpdSheetCell::Text(x)) => println!("the text is: {x}"),
        None => println!("the pop ele is: None")
    }

    for i in &rw {
        match i {
            SpdSheetCell::Int(x) => println!("the int is: {x}"),
            SpdSheetCell::Float(x) => println!("the float is: {x}"),
            SpdSheetCell::Text(x) => println!("the text is: {x}"),
        }
    }

}

enum SpdSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}