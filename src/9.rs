fn main() {
    let mut rng = rand::thread_rng();
    let num1: i32 = rng.gen_range(1..=10);
    let num2: i32 = rng.gen_range(1..=10);
    let answer = num1 * num2;
    println!("What is {} multiplied by {}?", num1, num2);
}
