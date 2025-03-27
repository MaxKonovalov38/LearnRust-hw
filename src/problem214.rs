use std::io;

/// Задача № 2.1 - "Среднее арифметическое трех чисел": Пользователь вводит три числа (дробные или целые).
/// Найдите их среднее арифметическое (сумма всех чисел, поделенная на их количество) и выведите результат на экран.
/// Sample Input: 1
///               2
///               3
/// Sample Output: 2

pub fn run() {
    let mut str_num_one = String::new();
    io::stdin().read_line(&mut str_num_one)
        .expect("Error!");
    let number_one: f64 = str_num_one.trim().parse().unwrap();
    
    let mut str_num_two = String::new();
    io::stdin().read_line(&mut str_num_two)
        .expect("Error!");
    let number_two: f64 = str_num_two.trim().parse().unwrap();
    
    let mut str_num_three = String::new();
    io::stdin().read_line(&mut str_num_three)
        .expect("Error!");
    let number_three: f64 = str_num_three.trim().parse().unwrap();
    
    let half_num = (number_one + number_two + number_three) / 3.0;
    
    println!("{}", half_num);
}