# Enums and Pattern Matching

## Defining an Enum

For example

```rs
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

This enum has four variants with different types:

- `Quit` has no data associated with it at all.

- `Move` includes an anonymous struct inside it.

- `Write` includes a single `String`.

- `ChangeColor` includes three `i32` values.
