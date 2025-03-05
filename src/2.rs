fn main() {
    let mut rng = rand::thread_rng();
    let number: u32 = rng.gen_range(1..=50);
    println!("The random number is: {}", number);
}
