# Using Structs to Structure Related Data

## Method Syntax

**Method** are similar to functions: they're declared with the `fn` keyword and their name, they can have parameters and a return value, and they contain some code that is run when they're called from somewhere else. However, methods are different from functions in that they're defined within the context of a struct(or an enum or a trait object), and their first parameter is always `self`, which represents the instance of the struct the method is being called on.

> Notice the head before struct `#[derive(Debug)]`

```rs
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the Rectangle is {} square pixels.", rect1.area());
}
```

Another useful feature of `impl` blocks is that we're allowed to define functions within `impl` blocks that don't take `self` as a parameter. These are called `associated functions` because they're associated with the struct. They're still functions, not methods, because they don't have an instance of the struct to work with. You've already used the `String::from` associated function.

Associated functions are often used for constructors that will return a new instance of the struct.

## Multiple `impl` Blocks

Each strut is allowed to have multiple `impl` blocks.

```rs
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

There're no reason to separate these methods into multiple `impl` blocks here, but this is valid syntax.
