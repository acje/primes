use std::time::Instant;

fn main() {
    let now = Instant::now();
    const LIMIT: u64 = 100_000;
    let mut vec = vec![2, 3, 5, 7];
    let mut count = 4u64;
    let mut candidate = 11u64;
    let upper_divisor = (LIMIT as f64).sqrt() as u64 + 1;

    loop {
        if check_for_prime(candidate, &vec) {
            count += 1;
            if candidate < upper_divisor {
                vec.push(candidate);
            }
        }
        candidate += 2; // skip odd numbers
        if candidate >= LIMIT {
            break;
        }
        // second half of unroled loop
        if check_for_prime(candidate, &vec) {
            count += 1;
            vec.push(candidate);
        }
        candidate += 4; // skip every second odd number as it divides by 3
        if candidate >= LIMIT {
            break;
        }
    }
    println!(
        "All primes smaller than {0} computed in {1} microseconds",
        LIMIT,
        now.elapsed().as_micros()
    );
    println!("Count: {}", count);
}

fn check_for_prime(pc: u64, vec: &[u64]) -> bool {
    let upper_limit = (pc as f64).sqrt() as u64 + 1;
    for element in vec.iter() {
        if pc % element == 0 {
            return false;
        }
        if element >= &upper_limit {
            return true;
        }
    }
    return true; // this will never happen.
}
