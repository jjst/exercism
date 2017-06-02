
pub fn sum_of_multiples(upper_bound: u16, numbers: &Vec<u16>) -> u64 {
    let mut total_sum: u64 = 0;
    for n in 1..upper_bound {
        if numbers.iter().any(|&x| n % x == 0) {
            total_sum += n as u64;
        }
    }
    total_sum
}
