use std::iter;

pub fn encode(s: &str) -> String {
    match s.chars().next() {
        Some(first_char) => {
            let count = s.chars().take_while(|&c| c == first_char).count();
            let rest: String = s.chars().skip(count).collect();
            let mut encoded = String::with_capacity(1);
            if count > 1 {
                encoded.push_str(&count.to_string());
            }
            encoded.push(first_char);
            let encoded_rest = encode(&rest);
            encoded.push_str(&encoded_rest);
            encoded
        }
        None => String::new()
    }
}

pub fn decode(s: &str) -> String {
    if s.is_empty() {
        String::new()
    } else {
        let digit_string: String = s.chars().take_while(|c| c.is_digit(10)).collect();
        let num_occurences = digit_string.parse::<u8>().unwrap_or(1);
        let mut chars = s.chars();
        let char_to_repeat = chars.nth(digit_string.len()).unwrap();
        let mut decoded: String = iter::repeat(char_to_repeat).take(num_occurences as usize).collect();
        let rest: String = chars.collect();
        let decoded_rest = decode(&rest);
        decoded.push_str(&decoded_rest);
        decoded
    }
}

