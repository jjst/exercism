pub fn build_proverb(pieces: Vec<&str>) -> String {
    let mut sentences: Vec<String> =
        pieces
            .iter()
            .zip(pieces.iter().skip(1))
            .map(|(w1, w2)| format!("For want of a {} the {} was lost.", w1, w2))
            .collect();
    if pieces.len() >= 3 {
        sentences.push(format!("And all for the want of a {}{} {}.", pieces[2], pieces[1], pieces[0]));
    } else if pieces.len() >= 1 {
        sentences.push(format!("And all for the want of a {}.", pieces[0]));
    }
    sentences.join("\n")
}
