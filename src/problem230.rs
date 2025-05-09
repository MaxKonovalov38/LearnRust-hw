use std::io;

/// Задача № 2.3 - Дано целое число. Выведите четные числа от 1 до n
/// Sample Input: 20
/// Sample Output: 2
///                4
///                6
///                8
///               10
///               12
///               14
///               16
///               18
///               20

pub fn run() {
    let mut str_num_integer = String::new();
    io::stdin().read_line(&mut str_num_integer)
        .expect("Error!");
    let number_int: i32 = str_num_integer.trim().parse().unwrap();
    
    let mut int_num = 1;
    
    while int_num <= number_int {
        if int_num % 2 == 0 {
            println!("{}", int_num);
        }
        
        int_num += 1;
    }
}