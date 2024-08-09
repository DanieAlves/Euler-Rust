fn sum_square_difference() -> u64 {
    let sum_of_squares = (1..101).map(|x| x * x).sum::<u64>();
    let square_of_sum = (1..101).sum::<u64>().pow(2);
    square_of_sum - sum_of_squares
}

fn main() {
    println!("{}", sum_square_difference());
}
