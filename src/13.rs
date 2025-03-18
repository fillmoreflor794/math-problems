use rand::prelude::*;

fn generate_random_math_problem(min: i32, max: i32) -> String {
    let operator = vec!["+", "-", "*"];
    let random_operator = operator.choose(&mut rand::thread_rng()).unwrap();
    let random_number = rand::thread_rng().gen_range(min..max);
    return format!("{} {} {} = ", random_number, random_operator, min);
}
