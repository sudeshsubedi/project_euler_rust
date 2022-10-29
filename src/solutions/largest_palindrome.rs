
// A palindromic number reads the same both ways. The largest palindrome
// made from the product of two 2-digit numbers is 9009 = 91 × 99.
// Find the largest palindrome made from the product of two 3-digit numbers.

pub fn largest_palindrome() -> u64 {
    let mut largest: u64 = 0;
    for i in 100..=999 {
        for j in 100..=999 {
            let prod = i * j;
            if is_palindrome(prod) {
                largest = if largest < prod { prod } else { largest }
            }
        }
    }
    largest
}

fn is_palindrome(mut num: u64) -> bool {
    let mut rev: u64 = 0;
    let orig: u64 = num;
    while num > 0 {
        rev *= 10;
        rev += num % 10;
        num /= 10;
    }
    orig == rev
}