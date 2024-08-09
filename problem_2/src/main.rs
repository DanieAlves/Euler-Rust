fn main() {
    let exceed = 4_000_000;
    let mut sum = 0;
    let mut fib = (1, 2);

    while fib.1 < exceed {
        let mut indice = fib.1;
        sum += indice;
        fib.1 = fib.0 + fib.1;
        fib.0 = indice;
    }

    println!("sum of the even-valued terms: {}", sum);
}
