
pub fn factors(number: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut current = number;
    while current > 1 {
        for i in 2..(current + 1) {
            if current % i == 0 {
                current = current / i;
                factors.push(i);
                break;
            }
        }
    }
    factors
}
