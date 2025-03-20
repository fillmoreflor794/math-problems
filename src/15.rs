fn main() {
    let x = 3;
    let y = 5;
    println!("{} + {} = {}", x, y, add(x, y));
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
