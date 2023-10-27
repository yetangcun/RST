fn main() {
    // 整型
    let integer = 2i32;
    println!("the integer val is {}", integer);

    let integer = 222u8;
    println!("the new integer val is {}", integer);

    // 浮点型
    let floatval32:f32 = 16.8;
    println!("the f val is: {floatval32}");

    let floatval = 16.88f32;
    println!("the f32 val is: {}", floatval);

    let result:f32 = floatval-floatval32;
    println!("the result is: {}", result);

    // bool类型
    let bl_true = true;
    let bl_false:bool = false;
    println!("the bool vals is: {},{}", bl_true, bl_false);

    // 字符型
    let char_code = '+';
    let str_code = "lalala";
    println!("char is {char_code}, str is {str_code}");

    // 复合类型 元组
    let _tups = (16.88, 12, 9);
    let tup_1 = _tups.0;
    let tup_2 = _tups.1;
    let tup_3 = _tups.2;
    println!("tup vals are: {tup_1}, {tup_2}, {tup_3}");
    println!("tup vals are: {}, {}, {}", tup_1, tup_2, tup_3);

    // 数组类型
    let _weeks=["monday","tuesday","wednesday","thursday","friday","saturday","sunday"];
    let _arrs:[i32; 4] = [1,2,3,4]; //["a1","a2","a3","a4"];
    let week1 = _weeks[0];
    let arr1 = _arrs[1];
    println!("week1 val is:{week1}; arr1 val is:{arr1}");
}
