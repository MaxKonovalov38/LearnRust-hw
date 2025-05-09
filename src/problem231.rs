use std::io;

/// Задача № 2.3 - Дано целое число. Выведите таблицу умножения к числу n
/// Sample Input: 5
/// Sample Output: 5 x 1 = 5
///                5 x 2 = 10
///                5 x 3 = 15
///                5 x 4 = 20
///                5 x 5 = 25
///                5 x 6 = 30
///                5 x 7 = 35
///                5 x 8 = 40
///                5 x 9 = 45
///                5 x 10 = 50

pub fn run() {    
    let mut str_number = String::new();
    io::stdin().read_line(&mut str_number)
        .expect("Error!");
    let number: i32 = str_number.trim().parse().unwrap();

    let mut i: i32 = 1;
    
    loop {
        println!("{} x {} = {}", number, i, number*i);
        i += 1;
        if i > 10 {
            break;
        }
    }
}