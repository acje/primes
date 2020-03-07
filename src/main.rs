use std::time::Instant;

fn main() {
    let now = Instant::now();
    const LIMIT: u64 = 100_000;
    let mut vec = vec![2, 3, 5, 7];
    let mut _count = 4u64;
    let mut _candidate = 11u64;
    let _upper_divisor = (LIMIT as f64).sqrt() as u64 + 1;

    loop {
        if check_for_prime(_candidate, &vec) {
            _count += 1;
            if _candidate < _upper_divisor {
                vec.push(_candidate);
            }
        }
        _candidate += 2; // skip odd numbers
        if _candidate >= LIMIT {
            break;
        }
        // second half of unroled loop
        if check_for_prime(_candidate, &vec) {
            _count += 1;
            vec.push(_candidate);
        }
        _candidate += 4; // skip every second odd number as it divides by 3
        if _candidate >= LIMIT {
            break;
        }
    }
    println!(
        "All primes smaller than {0} computed in {1} seconds",
        LIMIT,
        now.elapsed().as_secs()
    );
    println!("Count: {}", _count);
}

fn check_for_prime(_pc: u64, vec: &[u64]) -> bool {
    let _upper_limit = (_pc as f64).sqrt() as u64 + 1;
    for element in vec.iter() {
        if _pc % element == 0 {
            return false;
        }
        if element >= &_upper_limit {
            return true;
        }
    }
    return true; // this will never happen.
}
