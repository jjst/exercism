use std::collections::HashSet;

pub fn sum_of_multiples(upper_bound: u16, numbers: &Vec<u16>) -> u64 {
    let mut multiples = HashSet::new();
    for &n in numbers {
        let mut i = n;
        while i < upper_bound {
            multiples.insert(i);
            i += n;
        }
    }
    multiples.iter().fold(0, |sum, &x| sum + (x as u64))
}
