
use std::fmt;

struct Circle {
    radius: f64,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Circle with radius {}", self.radius)
    }
}

fn main() {
    let circle = Circle { radius: 5.0 };
    println!("{}", circle);
}