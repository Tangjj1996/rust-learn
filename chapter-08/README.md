# Generic Type, Traits, and Lifetimes

## Generic Data Types

```rs
struct Point<T, U> {
    x: T,
    y: U,
}
impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.y,
            y: other.y
        }
    }
}
```

The purpose of this examples is to demonstrate a situation in which some generic parameters are declare with `impl` and some are declared with the mothod definition. Here, the generic parameters `T` and `U` are declared after `impl`, because they go with the struct definition. The generic parameters `V` and `W` are declared after `fn mixup`, because they're only relevant to the method.

Notice, we set `self` rather than `&self` as above examples, because we except `T` parameters not `&T`.
