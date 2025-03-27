use std::io;

/// Задача № 2.1 - Напишите программу, которая считывает сторону квадрата и выводит его площадь.
/// Sample Input: 4
/// Sample Output: 16

pub fn run() {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Error!");
    let number: i32 = input.trim().parse().unwrap();
    println!("{}", number*number);
}