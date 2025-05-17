fn main() {
    // Example usage
    let num1 = 5;
    let num2 = 3;

    match (num1, num2) {
        (a, b) if a % 2 == 0 && b % 2 == 0 => println!("Both numbers are even.");
        _ => println!("At least one number is odd."),
    }
}
