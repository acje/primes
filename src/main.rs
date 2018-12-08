use std::time::Instant;

fn main() {
    let now = Instant::now();
    let _limit = 10_000_000u64;
    let mut vec = vec![2, 3];
    let mut _count = 2u64;
    let mut _candidate = 5u64;
    let _upper_divisor = (_limit as f64).sqrt() as u64 + 1;

    loop {
        if check_for_prime(_candidate, &vec) {
            _count += 1;
            if _candidate < _upper_divisor {
                vec.push(_candidate);
            }
        }
        _candidate += 2; // skip odd numbers
        if _candidate >= _limit {
            break;
        }
        // second half of unroled loop
        if check_for_prime(_candidate, &vec) {
            _count += 1;
            vec.push(_candidate);
        }
        _candidate += 4; // skip every second odd number as it devides by 3
        if _candidate >= _limit {
            break;
        }
    }
    println!(
        "All primes smaller than {0} computed in {1} seconds",
        _limit,
        now.elapsed().as_secs()
    );
    println!("Count: {}", _count);
}

fn check_for_prime(_pc: u64, vec: &[u64]) -> bool {
    let _upper_limit = (_pc as f64).sqrt() as u64 + 1;
    let mut vec_pos = 0usize;
    loop {
        if _pc % *&vec[vec_pos] == 0 {
            return false;
        }
        if *&vec[vec_pos] >= _upper_limit {
            return true;
        }
        vec_pos += 1;
    }
}
