# Syntax Detail

- `String::new`, `String` is a string type provided by the standart library, `::new` indicates that `new` is an associated function of the `String` type(an associated function is implemented on a type).

- `use std::io` put line at the beginning of the program is equal to `std::io::stdin` while there is just `io::stdin`, the standard library is a type that represents a handle to the standard input for your terminal.

- if `cargo update` doesn't work, just modify `Cargo.toml` by yourself.

- `match guess.trim().parse() { *** }`, `parse()` retruns a `Result` type and `Result` is an enum that has the variants `Ok` or `Err`. notice the `continue` flag will skip current loop for the parent scope.

- `loop` put all logic to loop.
