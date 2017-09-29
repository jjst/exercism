pub fn find() -> Option<u32> {
    pythagorean_triplet_summing_to(1000).map(|(a, b, c)| a * b * c)
}

pub fn pythagorean_triplet_summing_to(sum: u32) -> Option<(u32, u32, u32)> {
    triplets_summing_to(sum)
        .find(|&(a, b, c)| is_pythagorean_triplet(a, b, c))
}

pub struct TripletsSummingTo {
    curr: (u32, u32, u32),
    sum: u32,
}

impl Iterator for TripletsSummingTo {
    type Item = (u32, u32, u32);
    
    fn next(&mut self) -> Option<(u32, u32, u32)> {
        let (a, b, c) = self.curr;
        if b > a + 1 {
            self.curr = (a + 1, b - 1, c);
            Some((a, b, c))
        } else if c > b {
            self.curr = (1, self.sum - c, c - 1);
            Some((a, b, c))
        } else {
            None
        }
    }
}

pub fn triplets_summing_to(sum: u32) -> TripletsSummingTo {
    TripletsSummingTo { curr: (1, 1, sum - 2), sum: sum }
}

pub fn is_pythagorean_triplet(a: u32, b: u32, c: u32) -> bool {
    a*a + b*b == c*c
}

