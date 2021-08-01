fn main() {
    let _x = 5;

    let y = {
        let _x = 3;
        _x + 1
    };

    println!("The value of y is: {}", y);
}
