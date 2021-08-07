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

There two main way tansfer variable with `move` or `copy`

## References and Borrowing

```rs
fn calculate_length(s: &String) -> usize {
    s.len()
}
```

The scope in which the variable `s` is valid is the same as any function parameter's scope, but we don't drop what the refenrence points to when it goes out of scope because we don't have ownership. When functions have refenrences as parameters instead of the actual values, we don't need the return the values in order to give back ownership, because we never had ownership.

We call having refenrences as function parameters **borrowing**. As in real life, if a person owns something, you can borrow it from them. When you're done, you have to give it back.

Just as variables are immutable by default, so are references. We're not allowed to modify something we have a reference to.

But we can fix the error

```rs
fn main() {
    let mut s = String::from("hello")

    change(&mut s)
}

fn change(some_string: &mut String) {
    some_string.push_str(", world")
}
```

First, we had to change `s` to be `mut`. Then we had to create a mutable reference with `&mut s` and accept a mutable reference with `some_string: &mut String`.

But mutable references have one big restriction: you can have only one mutable refenrence to a particular piece of data in a particular scope.

As always, we can use curly brackets to create a new scope, allowing for multiple mutable references, just not **simultaneous** ones:

```rs
let mut s = String::from("hello")

{
    let r1 = &mut s
} // r1 goes out of scope here, so we can make a new reference with no problems.
let r2 = &mut s
```
