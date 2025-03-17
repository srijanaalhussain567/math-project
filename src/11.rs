
// Rust code for generating a random number between 1 and 6
fn main() {
    let mut rng = rand::thread_rng();
    let random_number: u8 = rng.gen_range(1, 7);
    println!("The random number is {}", random_number);
}