
pub fn fmt_bottle_count(bottle_count: u8) -> String {
    if bottle_count > 1 {
        format!("{} bottles", bottle_count)
    } else if bottle_count == 1 {
        "1 bottle".to_string()
    } else {
        "no more bottles".to_string()
    }
}
pub fn verse(bottle_count: u8) -> String {
    if bottle_count > 0 {
        let current_bcount = fmt_bottle_count(bottle_count);
        let new_bcount = fmt_bottle_count(bottle_count - 1);
        format!(
            "{} of beer on the wall, {} of beer.\nTake {} down and pass it around, {} of beer on the wall.\n",
            current_bcount,
            current_bcount,
            if bottle_count == 1 { "it" } else { "one" },
            new_bcount
        )
    } else {
        "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string()
    }
}

pub fn sing(start: u8, end: u8) -> String {
    let verses = (end..start + 1).rev().map(verse).collect::<Vec<String>>();
    verses.join("\n")
}
