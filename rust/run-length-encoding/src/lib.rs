use std::iter;

pub fn encode(to_encode: &str) -> String {
    if to_encode.is_empty() {
        String::new()
    } else {
        let mut encoded = String::with_capacity(to_encode.len());
        let mut chars = to_encode.chars();
        let mut previous_char = chars.next().unwrap();
        let mut num_occurences = 1;
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

