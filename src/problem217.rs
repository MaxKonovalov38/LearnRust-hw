use std::io;

/// Задача № 2.1 - Дано двухзначное число. Выведите его первую цифру (число десятков)
/// Sample Input: 40
/// Sample Output: 4

pub fn run() {
    let mut str_num_first = String::new();
    io::stdin().read_line(&mut str_num_first)
        .expect("Error!");
    let number_two: i32 = str_num_first.trim().parse().unwrap();
    
    println!("{}", number_two/10);
}