pub fn is_armstrong_number(num: u32) -> bool {
    let digits = digits(num);
    let num_digits = digits.len();
    let sum: u32 = digits
        .iter()
        .map(|&d| (d as u32).pow(num_digits as u32))
        .sum();
    sum == num
}

fn digits(number: u32) -> Vec<u8> {
    let mut digits = Vec::new();
    let mut n = number;
    while n > 0 {
        digits.push((n % 10) as u8);
        n /= 10;
    }
    digits
}
