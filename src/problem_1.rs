fn main() {
    let n = 1;
    let mut sum = 0;

    for i in n..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    println!("sum of all the multiples of 3 or 5 below 1000 is: {}", sum);
}
