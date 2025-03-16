use std::collections::HashSet;

fn main() {
    let mut set = HashSet::new();
    for i in 0..100 {
        set.insert(i);
    }
    println!("{:?}", set);
}
