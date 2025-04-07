// problem 1: A + B = C
fn main() {
    let a: isize; 
    let b: isize; 
    let c: isize;
    
    println!("Enter values for a and b:");
    std::io::stdin().read_line(&mut a);
    std::io::stdin().read_line(&mut b);
    a = a.trim(); // remove leading space
    b = b.trim(); // remove leading space
    
    if a == "0" || b == "0" {
        println!("Invalid values. Please enter valid numbers.");
    } else {
        c = a.to_string() + b.to_string();
        println!("C is: {}", c);
    }
}
