fn main() {
    println!("{}", math_problem());
}

fn math_problem() -> u64 {
    let mut rng = rand::thread_rng();
    let problem = rng.gen_range(1..10);
    return problem;
}
