use std::io;

/// Задача № 2.1 - "Вычисление остатка от деления": Пользователь вводит два целых числа - ДЕЛИМОЕ и ДЕЛИТЕЛЬ.
/// Вычислите остаток от деления делимого на делитель и выведите результат на экран.
/// Sample Input: 5
///               7
/// Sample Output: 5

pub fn run() {
    let mut str_num_first = String::new();
    io::stdin().read_line(&mut str_num_first)
        .expect("Error!");
    let num_divisible: i32 = str_num_first.trim().parse().unwrap();
    
    let mut str_num_second = String::new();
    io::stdin().read_line(&mut str_num_second)
        .expect("Error!");
    let num_divider: i32 = str_num_second.trim().parse().unwrap();
    
    let incomplete_private = num_divisible / num_divider;
    let remainder_division = num_divisible - (num_divider * incomplete_private);
    
    println!("{}", remainder_division);
}