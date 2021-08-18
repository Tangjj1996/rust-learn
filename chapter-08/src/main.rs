mod lib;
use lib::{NewsArticle, Summary};

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "hello", y: 'c' };
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let article = NewsArticle {
        headline: String::from("first"),
        location: String::from("second"),
        author: String::from("third"),
        content: String::from("fourth"),
    };

    println!("This is just test {}", article.summarize())
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
