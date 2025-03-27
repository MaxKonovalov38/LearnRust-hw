use std::io;

/// Задача № 2.2 - Даны три натуральных числа A, B, C. Определите, существует ли треугольник с такими сторонами.
/// Если треугольник существует, выведите строку YES, иначе выведет строку NO. Треугольник - это три точки,
/// не лежащие на одной прямой.
/// Sample Input: 5
///               12
///               13
/// Sample Output: YES

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
    
    if (number_one < number_two+number_three) && (number_two < number_one+number_three) && (number_three < number_one+number_two) {
        println!("YES");
    } else {
        println!("NO");
    }
}