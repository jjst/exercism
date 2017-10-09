
pub fn factors(number: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut current = number;
    while current > 1 {
        let next_factor = smallest_divisor(current);
        factors.push(next_factor);
        current /= next_factor;
    }
    factors
}

pub fn smallest_divisor(number: u64) -> u64 {
    for i in 2..(number + 1) {
        if number % i == 0 {
            return i
        }
    }
    number
}
