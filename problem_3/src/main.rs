fn largest_prime_factor(n: u64) -> u64 {
    let mut n = n;
    let mut factor = 2;
    while factor < n {
        if n % factor == 0 {
            n /= factor;
        } else {
            factor += 1;
        }
    }
    factor
}

fn main() {
    let n = 600851475143;
    println!("{}", largest_prime_factor(n));
}
