fn main() {
    println!("Hello, world!");
    let s = String::from("hello world");
    let index = first_word(&s);
    println!("this is the index:{:?}", (index, 333),)
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // convert our `String` to an array of bytes

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
