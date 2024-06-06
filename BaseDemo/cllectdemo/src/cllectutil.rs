pub fn getMax(vec1: &Vec<i32>) {
    let mut vec = &vec1[0];
    for vl in vec1 {
        if vl > vec {
            vec = vl;
        }
    }
    println!("max:{vec}");
}

pub fn getMax0(vec1: &[i32]) {
    let mut vec = &vec1[0];
    for vl in vec1 {
        if vl > vec {
            vec = vl;
        }
    }
    println!("max:{vec}");
}