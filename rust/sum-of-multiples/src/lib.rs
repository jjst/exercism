
pub fn sum_of_multiples(upper_bound: u16, numbers: &Vec<u16>) -> u64 {
    let mut sum: u64 = 0;
    for &n in numbers {
        if n > upper_bound { break; }
        sum += n as u64;
    }
    sum
}
