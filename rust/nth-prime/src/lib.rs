pub fn nth(n: u8) -> Result<u32, String> {
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

fn is_prime(number: u32) -> bool {
   (2..number).all(|x| number % x != 0)
}
