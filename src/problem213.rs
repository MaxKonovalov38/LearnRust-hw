use std::io;

/// Задача № 2.1 - "Конвертер температур": Пользователь вводит температуру в градусах Цельсия (дробное число).
/// Переведите эту температуру в градусы Фаренгейта, используя формулу F = (C * 9 / 5) + 32,
/// где F - температура в Фаренгейтах, С - температура в Цельсиях, и выведите результат на экран.
/// Sample Input: 25
/// Sample Output: 77

pub fn run() {
    let mut str_num_first = String::new();
    io::stdin().read_line(&mut str_num_first)
        .expect("Error!");
    let num_cel: f64 = str_num_first.trim().parse().unwrap();
    
    let num_farth = (num_cel * 9.0/5.0) + 32.0;
    println!("{}", num_farth);
}