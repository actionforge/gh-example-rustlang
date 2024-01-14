use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let random_number: i32 = rng.gen_range(1..11);
    println!("Random number: {}", random_number);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_number() {
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let random_number: i32 = rng.gen_range(1..11);
            assert!(random_number >= 1 && random_number <= 10, "Random number out of range: {}", random_number);
        }
    }
}
