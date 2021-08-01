use std::io;

fn main() {
    loop {
        println!("please enter your fibonacci nth: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("faild to read line");
        if guess.trim() == "quit" {
            break;
        };
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("you fibonacci list is:: {}", fibonacci(guess));
        println!("please enter ctrl+c or put \'quit\' to end")
    }
}

fn fibonacci(_nth: i32) -> i32 {
    match _nth {
        0 => 1,
        1 => 1,
        _ => fibonacci(_nth - 1) + fibonacci(_nth - 2),
    }
}
