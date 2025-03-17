 fn main() {
     let x = 10;
     let y = 20;

     println!("{} + {} = {}", x, y, sum(x, y));
 }

 fn sum(a: i32, b: i32) -> i32 {
     a + b
 }