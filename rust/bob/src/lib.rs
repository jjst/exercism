
pub fn reply(sentence: &str) -> String {
    if sentence.ends_with("?") {
        "Sure.".to_string()
    } else if sentence.is_empty() {
        "Fine. Be that way!".to_string()
    } else if sentence == sentence.to_uppercase() {
        "Whoa, chill out!".to_string()
    } else {
        "Whatever.".to_string()
    }
}

