use std::io;

/// Задача № 2.1 - Дано целое неотрицательное число N, определите число десятков в нем (предпоследнюю цифру числа).
/// Если предпоследней цифры нет, то можно считать, что число десятков равно нулю. Числа 0 < N < 10^19.
/// Sample Input: 444
/// Sample Output: 4

pub fn run() {
    let mut str_num_one = String::new();
    io::stdin().read_line(&mut str_num_one)
        .expect("Error!");
    let number_one: u128 = str_num_one.trim().parse().unwrap();

    if number_one < 10 {
        println!("0");
    } else {
        let num_10 = number_one / 10 % 10;
        println!("{}", num_10);
    }
}