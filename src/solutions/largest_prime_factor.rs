// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143 ?


pub fn lpf(mut num: u64) -> u64 {
    if num <= 1 {
        return num;
    }
    let mut largest: u64 = 2;
    while num > largest {
        if num % largest == 0 {
            num /= largest;
        } else {
            largest += 1;
        }
    }
    largest
}