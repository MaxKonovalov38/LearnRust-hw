use std::io;

/// Задача № 2.2 - Напишите программу, которая решает уравнение ax^2 + bx + c = 0 и выводит корни на экран
/// с использованием условных операторов. Обработайте все возможные случаи: два вещественных корня
/// (вывести в порядке убывания), один вещественный корень, нет корней.
/// Sample Input: 1
///               -5
///               6
/// Sample Output: 3 2

fn square(x: i32) -> i32 {
    x * x
}

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

    if number_one == 0 {
        if number_two == 0 {
            if number_three == 0 {
                println!("Бесконечное множество решений");
            } else {
                println!("Нет корней");
            }
        } else {
            let x_num: f64 = -(number_three as f64) / (number_two as f64);
            println!("{}", x_num);
        }
        return;
    }

    let d_num: f64 = (square(number_two) - 4 * number_one * number_three) as f64;

    if d_num > 0.0 {
        let d_sqrt: f64 = d_num.sqrt();
        let x_one: f64 = (-(number_two as f64) + d_sqrt) / (2.0 * (number_one as f64));
        let x_two: f64 = (-(number_two as f64) - d_sqrt) / (2.0 * (number_one as f64));

        if x_one > x_two {
            println!("{} {}", x_one, x_two);
        } else {
            println!("{} {}", x_two, x_one);
        }
    } else if d_num == 0.0 {
        let x_num: f64 = -(number_two as f64) / (2.0 * number_one as f64);

        println!("{}", x_num);
    } else {
        println!("Нет корней");
    }
}