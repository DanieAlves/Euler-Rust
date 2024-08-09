fn prime(n: u64) -> bool {
    if n == 2 {
        return true;
    }
    if n < 2 || n % 2 == 0 {
        return false;
    }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

fn st_prime() -> u64 {
    let mut count = 1;
    let mut i = 3;
    while count < 10001 {
        if prime(i) {
            count += 1;
        }
        i += 2;
    }
    i - 2
}

fn main() {
    println!("{}", st_prime());
}
