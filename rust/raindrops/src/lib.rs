pub fn raindrops(number: u32) -> String {
    let mut r = String::new();; //String::from("Hello, ");
    if number % 3 == 0 {
        r.push_str("Pling");
    }
    if number % 5 == 0 {
        r.push_str("Plang");
    }
    if number % 7 == 0 {
        r.push_str("Plong");
    }
    if r.is_empty() {
        number.to_string()
    } else {
        r
    }
}

