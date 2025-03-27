use std::io;

/// Задача № 2.1 - Пользователь вводит цену товара (дробное число) и количество товара (целое число). Найдите общую стоимость покупки и выведите результат на экран.
/// Sample Input: 10
///               5
/// Sample Output: 50

pub fn run() {
    let mut str_num_first = String::new();
    io::stdin().read_line(&mut str_num_first)
        .expect("Error!");
    let number_one: f32 = str_num_first.trim().parse().unwrap();
    
    let mut str_num_second = String::new();
    io::stdin().read_line(&mut str_num_second)
        .expect("Error!");
    let number_two: i32 = str_num_second.trim().parse().unwrap();

    let num_as: f32 = number_two as f32;
    
    println!("{}", num_as*number_one);
}