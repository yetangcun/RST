pub mod traitimpl;
pub mod lifeutil;

/* 原始写法 */
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

/* 函数提取1 */
fn get_largest_num(arr: &[i32]) -> &i32 {
    let mut largest_num = &arr[0];
    for num in arr {
        if num > largest_num {
            largest_num = num;
        }
    }
    largest_num
}

fn get_largest_chr (lst:&[char]) -> &char {
    let mut largest_chr = &lst[0];
    for chr in lst {
        if chr > largest_chr {
            largest_chr = chr;
        }
    }
    largest_chr
} 

// fn get_largest<T> (lst:&[T]) -> &T {
//     let mut largest = &lst[0];
//     for itm in lst {
//         if itm > largest {
//             largest = itm;
//         }
//     }
//     largest
// }

pub fn largest_num1 () {
    let num_arr1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let largest_num1 = get_largest_num(&num_arr1);
    println!("the largest num:{largest_num1}");

    let num_arr2 = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    let largest_num2 = get_largest_num(&num_arr2);
    println!("the largest num:{largest_num2}");

    let chr_arr1 = vec!['a', 'b', 'd', 'e', 'c', 'h', 'j', 'f', 'i', 'g'];
    let largest_chr1 = get_largest_chr(&chr_arr1);
    println!("the largest chr:{largest_chr1}");

    let p_obj = Point { x: 1.0, y: 2.0 };
    let p1_obj = Point { x: "yes", y: 'N' };
    let p_new = p_obj.get_mix_point(p1_obj);
    println!("the mix point:({},{})", p_new.x, p_new.y);
}
/* 函数提取1 */

/* 函数提取2 */

/* 函数提取2 */

pub struct Point<T1,T2> {
    x:T1,
    y:T2
}

impl<T1,T2> Point<T1,T2> {
    fn get_mix_point<T3,T4>(self, oth: Point<T3,T4>) -> Point<T1,T4> {
        Point {
            x:self.x,
            y:oth.y
        }
    }
}