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
    let mut chars = s.chars();
    match chars.next() {
        Some(next_char) => {
            let mut decoded: String = match next_char.to_digit(10) {
                Some(num_occurences) => {
                    iter::repeat(chars.next().unwrap()).take(num_occurences as usize).collect()
                }
                None => next_char.to_string()
            };
            let rest: String = chars.collect();
            let decoded_rest = decode(&rest);
            decoded.push_str(&decoded_rest);
            decoded
        }
        None => String::new()
    }
}

