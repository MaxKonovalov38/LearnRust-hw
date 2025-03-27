use std::io;

/// Задача № 2.2 - Входные данные: Часы (целое число от 0 до 23). УТРО [6-11), ДЕНЬ [11-17), ВЕЧЕР [17-22),
/// НОЧЬ [22-6). Выходные данные: "Утро", "День", "Вечер" или "Ночь" в зависимости от времени суток.
/// Sample Input: 6
/// Sample Output: Утро

pub fn run() {
    let mut str_num_integer = String::new();
    io::stdin().read_line(&mut str_num_integer)
        .expect("Error!");
    let number_integer: i32 = str_num_integer.trim().parse().unwrap();
    
    match number_integer {
        6..=10 => println!("Утро"),
        11..=16 => println!("День"),
        17..=21 => println!("Вечер"),
        22..=24 | 1..=5 => println!("Ночь"),
        _ => println!("Error!"),
    }
}