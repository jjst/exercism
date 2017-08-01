pub fn encode(to_encode: &str) -> String {
    let mut chars = to_encode.chars();
    match chars.next() {
        None => String::new(),
        Some(first_char) => {
            let mut previous_char = first_char;
            let mut num_occurences = 1;
            let mut encoded = String::with_capacity(to_encode.len());
            for chr in chars {
                if chr == previous_char {
                    num_occurences += 1;
                } else {
                    if num_occurences > 1 {
                        encoded.push_str(&num_occurences.to_string());
                    }
                    encoded.push(previous_char);
                    num_occurences = 1;
                }
                previous_char = chr;
            }
            if num_occurences > 1 {
                encoded.push_str(&num_occurences.to_string());
            }
            encoded.push(previous_char);
            encoded
        }
    }
}

pub fn decode(s: &str) -> String {
    if s.is_empty() {
        String::new()
    } else {
        let mut decoded: String = String::with_capacity(s.len());
        let mut to_decode = String::from(s);
        while !to_decode.is_empty() {
            to_decode = {
                let digit_string: String = to_decode.chars().take_while(|c| c.is_digit(10)).collect();
                let num_occurences = digit_string.parse::<u8>().unwrap_or(1);
                let mut chars = to_decode.chars();
                let char_to_repeat = chars.nth(digit_string.len()).unwrap();
                for _ in 0..num_occurences {
                    decoded.push(char_to_repeat);
                }
                chars.collect()
            }
        }
        decoded
    }
}
