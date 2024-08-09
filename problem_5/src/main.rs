fn gcd(a: u64, b: u64) -> u64 {
    let mut x = a;
    let mut y = b;
    while y != 0 {
        let temp = y;
        y = x % y;
        x = temp;
    }
    x
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn smallest_multiple() -> u64 {
    let mut multiple = 1;
    for i in 1..21 {
        multiple = lcm(multiple, i);
    }
    multiple
}
fn main() {
    println!("{}", smallest_multiple());
}
