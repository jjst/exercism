
pub fn square_of_sum(upper_bound: u16) -> u64 {
    let sum = (1..(upper_bound + 1) as u64).fold(0, |res, acc| res + acc);
    sum*sum
}

pub fn sum_of_squares(upper_bound: u16) -> u64 {
    (1..(upper_bound + 1) as u64).fold(0, |res, acc| res + acc*acc)
}

pub fn difference(upper_bound: u16) -> u64 {
    square_of_sum(upper_bound) - sum_of_squares(upper_bound)
}
