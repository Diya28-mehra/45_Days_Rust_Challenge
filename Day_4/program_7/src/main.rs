use std::fmt::Display;
use std::cmp::PartialOrd;
fn main() {
    let num_list = vec![34,25,25,76,858,23];
    let largest = get_largest(num_list.clone());
    println!("The largest number in a vector is {}",largest);
    println!("{:?}",num_list);

    let char_list = vec!['a','g','d','h','s'];
    let largestc = get_largest(char_list);
    println!("{}",largestc);
}

fn get_largest<T:PartialOrd+Copy>(num_list:Vec<T>)->T{
    let mut largest = num_list[0];
    for num in num_list{
        if num>largest{
            largest = num;
        }
    }
    largest
}


// fn get_largest(num_list:Vec<i32>) -> i32{
//     let mut largest = num_list[0];
//     for num in num_list{
//         if num>largest{
//             largest = num;
//         }
//     }
//     largest
// }