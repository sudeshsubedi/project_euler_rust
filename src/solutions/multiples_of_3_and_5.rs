// If we list all the natural numbers below 10 that are multiples of 3 or 5,
// we get 3, 5, 6 and 9. The sum of these multiples is 23.
// Find the sum of all the multiples of 3 or 5 below 1000.


pub fn sum_of_multiple_of_three_five(upper: usize) -> usize {
    let mut sum: usize = 0;
    for i in 1..upper {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    sum
}