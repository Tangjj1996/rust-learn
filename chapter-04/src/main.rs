use std::io;

fn main() {
    println!("please enter your fibonacci nth: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("faild to read line");

    let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    println!("you fibonacci list is:: {}", fibonacci(guess));
}

fn fibonacci(_nth: i32) -> i32 {
    1
}
