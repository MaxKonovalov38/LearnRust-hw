use std::io;

/// Задача № 2.1 - Напишите программу, которая считывает два числа и выводит их сумму.
/// Sample Input: 1
///               2
/// Sample Output: 3

pub fn run() {
    let mut str_num_first = String::new();
    io::stdin().read_line(&mut str_num_first)
        .expect("Error!");
    let number_one: i32 = str_num_first.trim().parse().unwrap();
    
    let mut str_num_second = String::new();
    io::stdin().read_line(&mut str_num_second)
        .expect("Error!");
    let number_two: i32 = str_num_second.trim().parse().unwrap();
    
    println!("{}", number_one+number_two);
}