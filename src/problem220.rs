use std::io;

/// Задача № 2.2 - Входные данные - целое число. Выходные данные - "Четное" или "Нечетное".
/// Sample Input: 2
/// Sample Output: Четное

pub fn run() {
    let mut str_num_integer = String::new();
    io::stdin().read_line(&mut str_num_integer)
        .expect("Error!");
    let number_integer: i32 = str_num_integer.trim().parse().unwrap();
    
    if number_integer % 2 == 0 {
        println!("Четное");
    } else {
        println!("Нечетное");
    }
}