fn main() {
    let problem = gen_random_problem();
    println!("The problem is: {}", problem);
}

fn gen_random_problem() -> String {
    // Generate a random number between 1 and 10
    let num1 = rand::thread_rng().gen_range(1, 11);
    
    // Generate a random operation (+, -, x, /)
    let op = match rand::thread_rng().gen_range(0, 4) {
        0 => "+",
        1 => "-",
        2 => "*",
        3 => "/",
        _ => "error"
    };
    
    // Generate a random number between 1 and 10
    let num2 = rand::thread_rng().gen_range(1, 11);
    
    // Return the problem as a string
    format!("{} {} {} = ?", num1, op, num2)
}
