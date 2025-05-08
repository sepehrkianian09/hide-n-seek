use rand::Rng;
use rand::rngs::StdRng;
use rand::SeedableRng;

// A function that takes a trait object (Box<dyn Rng>) as a parameter
fn generate_random_number(rng: &mut dyn Rng) -> u32 {
    rng.gen_range(1..101)  // Generate a random number between 1 and 100
}

fn main() {
    // Create a concrete RNG type (StdRng) and seed it
    let mut rng: Box<dyn Rng> = Box::new(StdRng::seed_from_u64(42));

    // Pass the trait object to the function
    let random_number = generate_random_number(&mut rng);
    println!("Generated random number: {}", random_number);
}
