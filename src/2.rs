use std::thread;
use std::time;

fn main() {
    let start = time::Instant::now();

    // Perform some computation here
    for i in 0..10000000 {
        if i % 2 == 0 {
            println!("{}", i);
        }
    }

    let end = time::Instant::now();
    let elapsed = end.duration_since(start).as_secs_f64();
    println!("Time: {} seconds", elapsed);
}
