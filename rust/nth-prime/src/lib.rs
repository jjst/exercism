pub fn nth(n: u16) -> Result<u32, &'static str> {
    if n <= 0 {
        Err("n must be greater than 0")
    } else {
        let mut count = 0;
        let mut i = 2;
        loop {
            if is_prime(i) {
                count += 1;
                if count == n {
                   return Ok(i)
                }
            }
            i += 1;
        }
    }
}

fn is_prime(number: u32) -> bool {
   (2..((number as f32).sqrt() as u32 + 1)).all(|x| number % x != 0)
}
