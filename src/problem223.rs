use std::io;

/// Задача № 2.2 - Даны три угла треугольника (в градусах). Определите, в зависимости от углов треугольник -
/// "Равносторонний", "Равнобедренный", "Произвольный" или "Не существует".
/// Sample Input: 100
///               20
///               60
/// Sample Output: Произвольный

pub fn run() {
    let mut str_num_one = String::new();
    io::stdin().read_line(&mut str_num_one)
        .expect("Error!");
    let number_one: i32 = str_num_one.trim().parse().unwrap();
    
    let mut str_num_two = String::new();
    io::stdin().read_line(&mut str_num_two)
        .expect("Error!");
    let number_two: i32 = str_num_two.trim().parse().unwrap();
    
    let mut str_num_three = String::new();
    io::stdin().read_line(&mut str_num_three)
        .expect("Error!");
    let number_three: i32 = str_num_three.trim().parse().unwrap();
    
    let sum_num = number_one + number_two + number_three;
    
    if (number_one == 60) && (number_two == 60) && (number_three == 60) && sum_num == 180 {
        println!("Равносторонний");
    } else if ((number_one == number_two) || (number_one == number_three) || (number_two == number_three)) && sum_num == 180 {
        println!("Равнобедренный");
    } else if sum_num == 180 {
        println!("Произвольный");
    } else {
        println!("Не существует");
    }
}