mod solutions;

use solutions::largest_palindrome;

fn main() {
    // let num: u64 = 600_851_475_143;
    println!("largest palindrome: {}", largest_palindrome::largest_palindrome());
}
