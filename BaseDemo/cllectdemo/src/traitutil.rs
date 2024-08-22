pub fn larger_num() {
    let num_arr1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut largest_num1 = &num_arr1[0];
    for num in &num_arr1 {
        if num > largest_num1 {
            largest_num1 = num;
        }
    }
    println!("largest_num1:{largest_num1}");

    let num_arr2 = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    let mut largest_num2 = &num_arr2[0];
    for num in &num_arr2 {
        if num > largest_num2 {
            largest_num2 = num;
        }
    }
    println!("largest_num2:{largest_num2}");
}


fn get_largest_num(arr: &[i32]) -> &i32 {
    let mut largest_num = &arr[0];
    for num in arr {
        if num > largest_num {
            largest_num = num;
        }
    }
    largest_num
}

pub fn largest_num1 () {
    let num_arr1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let largest_num1 = get_largest_num(&num_arr1);
    println!("the largest num:{largest_num1}");

    let num_arr2 = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    let largest_num2 = get_largest_num(&num_arr2);
    println!("the largest num:{largest_num2}");
}
