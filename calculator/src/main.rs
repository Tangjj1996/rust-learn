use std::io::{stdin, stdout, Write};

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

fn main() {
    println!("Welcome to my calctor");
    loop {
        let mut first_param = String::new();
        let mut second_param = String::new();
        let mut operator = String::new();

        println!("Please enter your first param: ");
        read(&mut first_param);

        println!("Please enter your second param: ");
        read(&mut second_param);

        println!("Please enter your operator(+-*/): ");
        read(&mut operator);

        let first_param: i32 = first_param.trim().parse().unwrap();
        let second_param: i32 = second_param.trim().parse().unwrap();
        let operator: char = operator.trim().chars().next().unwrap();

        let operators = String::from("+-*/");

        if !operators.contains(operator) {
            println!("the operator is invalid, let's restart");
            continue;
        }

        let result = match operator {
            '+' => first_param + second_param,
            '-' => first_param - second_param,
            '*' => first_param * second_param,
            '/' => first_param / second_param,
            _ => panic!(""),
        };

        println!(
            "so the result is {} {} {} = {}",
            first_param, operator, second_param, result
        );
        break;
    }
}
