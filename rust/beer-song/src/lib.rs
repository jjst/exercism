pub fn fmt_bottle_count(bottle_count: u8, capitalize: bool) -> String {
    if bottle_count > 1 {
        format!("{} bottles", bottle_count)
    } else if bottle_count == 1 {
        "1 bottle".to_string()
    } else {
        format!("{}o more bottles", if capitalize { "N" } else { "n" })
    }
}
pub fn verse(bottle_count: u8) -> String {
    let new_bottle_count = if bottle_count > 0 { bottle_count - 1 } else { 99 };
    let what_to_do =
        if bottle_count > 0 {
            format!("Take {} down and pass it around", if bottle_count == 1 { "it" } else { "one" })
        } else {
            "Go to the store and buy some more".to_string()
        };
    format!(
        "{} of beer on the wall, {} of beer.\n{}, {} of beer on the wall.\n",
        fmt_bottle_count(bottle_count, true),
        fmt_bottle_count(bottle_count, false),
        what_to_do,
        fmt_bottle_count(new_bottle_count, false)
    )
}

pub fn sing(start: u8, end: u8) -> String {
    let verses = (end..start + 1).rev().map(verse).collect::<Vec<String>>();
    verses.join("\n")
}
