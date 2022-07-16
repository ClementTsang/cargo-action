use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    println!("Hello, world!");
    println!("A random number: {}", rng.gen::<u64>());
}
