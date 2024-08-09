fn largest_palindrome() -> u64 {
    let mut max = 0;
    let var_name = for i in 100..1000 {
        for j in 100..1000 {
            let product = i * j;
            if product.to_string() == product.to_string().chars().rev().collect::<String>() {
                if product > max {
                    max = product;
                }
            }
        }
    };
    max
}

fn main() {
    println!("{}", largest_palindrome());
}
