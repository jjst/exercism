extern crate beer_song as beer;

#[test]
fn test_fmt_bottle_count_0() {
    assert_eq!(beer::fmt_bottle_count(0, false), "no more bottles");
}

#[test]
fn test_fmt_bottle_count_0_capitalized() {
    assert_eq!(beer::fmt_bottle_count(0, true), "No more bottles");
}

#[test]
fn test_fmt_bottle_count_1() {
    assert_eq!(beer::fmt_bottle_count(1, false), "1 bottle");
}

#[test]
fn test_fmt_bottle_count_2() {
    assert_eq!(beer::fmt_bottle_count(2, false), "2 bottles");
}

#[test]
fn test_verse_0() {
    assert_eq!(beer::verse(0), "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
}

#[test]
fn test_verse_1() {
    assert_eq!(beer::verse(1), "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n");
}

#[test]
fn test_verse_2() {
    assert_eq!(beer::verse(2), "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n");
}

#[test]
fn test_verse_8() {
    assert_eq!(beer::verse(8), "8 bottles of beer on the wall, 8 bottles of beer.\nTake one down and pass it around, 7 bottles of beer on the wall.\n");
}

#[test]
fn test_verse_99() {
    assert_eq!(beer::verse(99), "99 bottles of beer on the wall, 99 bottles of beer.\nTake one down and pass it around, 98 bottles of beer on the wall.\n");
}

#[test]
fn test_song_8_6() {
    assert_eq!(beer::sing(8, 6), "8 bottles of beer on the wall, 8 bottles of beer.\nTake one down and pass it around, 7 bottles of beer on the wall.\n\n7 bottles of beer on the wall, 7 bottles of beer.\nTake one down and pass it around, 6 bottles of beer on the wall.\n\n6 bottles of beer on the wall, 6 bottles of beer.\nTake one down and pass it around, 5 bottles of beer on the wall.\n");
}

#[test]
fn test_song_3_0() {
    assert_eq!(beer::sing(3, 0), "3 bottles of beer on the wall, 3 bottles of beer.\nTake one down and pass it around, 2 bottles of beer on the wall.\n\n2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n\n1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n\nNo more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
}
