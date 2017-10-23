use std::cmp;

pub fn find() -> Option<u32> {
    pythagorean_triplet_summing_to(1000).map(|(a, b, c)| a * b * c)
}

pub fn pythagorean_triplet_summing_to(sum: u32) -> Option<(u32, u32, u32)> {
    triplets_summing_to(sum)
        .find(|&(a, b, c)| is_pythagorean_triplet(a, b, c))
}

pub struct TripletsSummingTo {
    curr: Option<(u32, u32, u32)>,
    sum: u32,
}

impl Iterator for TripletsSummingTo {
    type Item = (u32, u32, u32);
    
    fn next(&mut self) -> Option<(u32, u32, u32)> {
        let curr = self.curr;
        if let Some((a, b, c)) = self.curr {
            self.curr = 
                if a < b - 1 {
                    Some((a + 1, b - 1, c))
                } else if b < c - 1 {
                    let new_c = c - 1;
                    let new_b = cmp::min(new_c, self.sum - c);
                    let new_a = self.sum - new_c - new_b;
                    Some((new_a, new_b, new_c))
                } else {
                    None
                };
        }
        curr
    }
}

pub fn triplets_summing_to(sum: u32) -> TripletsSummingTo {
    TripletsSummingTo { curr: Some((1, 1, sum - 2)), sum: sum }
}

pub fn is_pythagorean_triplet(a: u32, b: u32, c: u32) -> bool {
    a*a + b*b == c*c
}
