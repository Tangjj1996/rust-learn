# Understanding Ownership

## Ownership Rules

- Each value in Rust has a variable that's called its **owner**.

- There can only be one owner at a time.

- When the owner goes out of scope. the value will be dropped.

## Here are some of the types that implement `Copy`

- All the integer types, such as `u32`.

- The Boolean type, `bool`, with values `true` and `false`.

- All the floating point types, such as `f64`.

- The character type, `char`.

- Tuples, if they only contain types that also implement `Copy`. For example, `(i32, i32)` implements `Copy`, but `(i32, String)` does not.

## Ownership and Functions

The semantics for passing a value to a function are similar to those for assigning a value to a variable. Passing a variable to a function will move or copy, just as assignment does.

```rust
fn main() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here
    let x = 5;          // x comes into scope

    makes_copy(x);      // x would move into the function,
                        // but i32 is Copy, so it's okay to still
                        // use x afterward
} // Here, x goes out of scope, the s. But because s's value was moved, nothing special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Noting special happens.
```
