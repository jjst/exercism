//
// See Rust Language Specific Instructions
// below normal exercise description.
//

pub fn encode(number: u32) -> String {
    let s = match number {
        0 => "zero",
        1 => "one",
        _ => "heh"
    };
    String::from(s)
}
