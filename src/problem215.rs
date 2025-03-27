use std::io;

/// Задача № 2.1 - "Конвертер времени": Пользователь вводит количество часов (целое число) и минут (целое число).
/// Переведите это время в минуты, используя формулу ОБЩЕЕ_КОЛИЧЕСТВО_МИНУТ = (КОЛИЧЕСТВО_ЧАСОВ * 60) + КОЛИЧЕСТВО_МИНУТ,
/// и выведите результат на экран.
/// Sample Input: 3
///               2
/// Sample Output: 182

pub fn run() {
    let mut str_num_first = String::new();
    io::stdin().read_line(&mut str_num_first)
        .expect("Error!");
    let number_hour: i32 = str_num_first.trim().parse().unwrap();
    
    let mut str_num_second = String::new();
    io::stdin().read_line(&mut str_num_second)
        .expect("Error!");
    let number_minute: i32 = str_num_second.trim().parse().unwrap();
    
    let number_minutes = (number_hour * 60) + number_minute;
    println!("{}", number_minutes);
}