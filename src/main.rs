mod solutions;


fn main() {
    let upper: usize = 1000;
    println!("sum of multiple of 3 and 5 below {}: {}", upper, solutions::multiples_of_3_and_5::sum_of_multiple_of_three_five(upper));
}
