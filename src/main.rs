use std::time::Instant;
use std::convert::TryInto;

fn main() {
    let now = Instant::now();
    const LIMIT: u64 = 100_000_00;
    let mut primes_divisor = vec![2, 3, 5, 7];
    primes_divisor.reserve(((LIMIT / 5000)).try_into().unwrap());
    println!("Primes table is starting with capasity {}",primes_divisor.capacity());
    let mut count = 4u64;
    let mut candidate = 11u64;
    let upper_divisor = (LIMIT as f64).sqrt() as u64 + 1;

    loop {
        if check_for_prime(candidate, &primes_divisor) {
            count += 1;
            if candidate < upper_divisor {
                primes_divisor.push(candidate);
            }
        }
        candidate += 2; // skip odd numbers
        if candidate >= LIMIT {
            break;
        }
        // second half of unroled loop
        if check_for_prime(candidate, &primes_divisor) {
            count += 1;
            if candidate < upper_divisor {
                primes_divisor.push(candidate);
            }
        }
        candidate += 4; // skip every second odd number as it divides by 3
        if candidate >= LIMIT {
            break;
        }
    }
    println!("Primes table is ending with capasity {}",primes_divisor.capacity());
    println!(
        "All primes smaller than {0} computed in {1} ms",
        LIMIT,
        now.elapsed().as_millis()
    );
    println!("Count: {}", count);
}

fn check_for_prime(pc: u64, primes_divisor: &[u64]) -> bool {
    let upper_limit = (pc as f64).sqrt() as u64 + 1;
    for element in primes_divisor.iter() {
        if pc % element == 0 {
            return false;
        }
        if element >= &upper_limit {
            return true;
        }
    }
    return true; // this will never happen.
}
