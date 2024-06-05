use std::collections::HashMap;
pub fn has_map_fn() {
    let mut hm = HashMap::new();
    hm.insert(String::from("KY1"), 11);
    hm.insert(String::from("KY6"), 66);
    hm.insert(String::from("KY8"), 88);
    hm.insert(String::from("KY9"), 99);

    for (key, value) in &hm {
        println!("key:{key},value:{value}");
    }
    
    let g_vl = hm.get(&String::from("KY6")).unwrap();
    println!("g_vl:{g_vl}");

    let mut hmp = HashMap::new();
    let f_kv = String::from("KY6");
    let f_vl = String::from("VL6");
    println!("kv:{f_kv},vl:{f_vl}");
    hmp.insert(f_kv, f_vl); // 所有权被移动到hmp中

    // 遍历HashMap
    let strs = String::from("hello world wonderful world");
    let mut hasmp = HashMap::new();
    for str in strs.split(' ') {
        let counts = hasmp.entry(str).or_insert(0);
        *counts+=1;
    }
    println!("hasmp:{:?},{strs}",hasmp)
}