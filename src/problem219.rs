use std::io;

/// Задача № 2.1 - Дано целое трезначное число. Найдите сумму его цифр.
/// Sample Input: 567
/// Sample Output: 18

pub fn run() {
    let mut str_number = String::new();
    io::stdin().read_line(&mut str_number)
        .expect("Error!");
    let number: i32 = str_number.trim().parse().unwrap();
    
    let num_sum = (number / 100) + (number % 100) / 10 + (number % 10);
    
    println!("{}", num_sum);
}