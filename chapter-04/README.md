# Common Pragramming

## 1. Variables & Mutability

- `variables` are immutable by default but you can decalre `mut` to convert to mutable.

- the most difference between `variable` and `constance` is `constance` always immutable but another can convert.

- rustaceans say that the first variable is `shadowed` by the scond, which means that the second variable's value is what appears when the variable is used. Notice, we can `shadow` a variable by using the same variable's name and repeating the use of `let` keyword.

## 2. Data Types

> Every value in Rust is of a certain **data type**, which tells Rust what kind of data is being specified so it konws how to work with that data. We'll look at two data type: **scalar** and **compound**

- Scalar Types

> Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.

- Compound Types

> compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

### Tuple Type

In addition to destructuring through pattern matching, we can access a tuple element directly by using a period(.) followed by the index of the value we want to access.

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

the program creates a tuple, `x`, and then makes new variables for each element by using their respective indices. As with most programming languages, the first index in a tuple is 0.

### Array Type

another way to have a collection of multiple values is with an **array**. Unlike a tuple, every element of an array must have **the same type**.

you would write an array's type by using square breckets, and within the brackets include the type of each element, a semicolon, and then the number of elements in the array.

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

Here, `i32` is the type of each element, after the semicolon, the number `5` indicates the array contains five elements.

Writing an array's type this way looks similar to an alternative syntax for initalizing an array: if you want to create an array that contains the same value for each element, you can specify the initial value, followed by a semicolon, and then the length of the array in square brackets.

```rust
let a = [3; 5]
```

The array named `a` will contain `5` elements that will all be set to the value `3` initally. This is the same as writing `let a = [3, 3, ]`

## 3. Functions

- in function signatures, you must declare the type of each parameter. This is a deliberate decision in Rust's design: requiring type annotation in function definitions means the compiler almost never needs you to use them elsewhere in the code to figure out what you mean.

- calling a function is an expression, calling a macro is an expressin, the block that we use to create new scopes, `{}`, is an expression.

```rust
fn main() {
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);
}
```

This expression:

```rust
{
    let x = 3;
    x + 1
}
```

is a block that, is this case, evaluates to `4`. That value gets bound to `y` as part of the `let` statement. Note the `x + 1` line without a semicolon at the end, which is unlike most of the lines you've seen so far. Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value. Keep this is in mind as you explore function return values and expressions next.

- Functions can return values to the code that calls them. We don't name return values, but we do declare their type after an arrow(`->`). In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function.

```rust
fn five() -> i32 { 5 }

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}
```
The `five` function has no parameters and defines the type of the return value, but the body of the function is a lonely `5` with no semicolon because it's an expression whose value we want to reutrn.

## Control Flow

- becase `if` is an expression, we can use it on the right side of a `let` statement.

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}
```

Remember that blocks of code evalute to the last expression in them, and numbers by themselves are also expressions. In this case, the value of the whole `if` expression depends on which block of code executes. This means the values that have the potential to be results from each arm of the `if` must be the same type.

- one of the uses of a `loop` is to retry an opration you konw might fail, such as checking whether a thread has completed its job. However, you might need to pass the result of that operation to the rest of your code. To do this, you can add the value you want returned after the `break` expression you use to stop the loop.

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}
```
