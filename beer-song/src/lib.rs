fn pluralize(n: u32, stem: &str) -> String {
    match n {
        0 => format!("no more {}s", stem),
        _ => format!("{} {}{}", &n, stem, if n > 1 { "s" } else { "" })
    }
}

pub fn verse(n: u32) -> String {
    if n == 0 {
        return "No more bottles of beer on the wall, no more bottles of beer.\n\
                Go to the store and buy some more, 99 bottles of beer on the wall.\n".to_owned()
    }
    let original = pluralize(n, "bottle");
    let now = pluralize(&n-1, "bottle");
    let take = match n {
        1 => "it",
        _ => "one"
    };
    format!("{} of beer on the wall, {} of beer.\n\
           Take {} down and pass it around, {} of beer on the wall.\n", original, original, take, now)
}

pub fn sing(start: u32, end: u32) -> String {
    let verses: Vec<String> = (end..start+1).rev().map(|num| verse(num)).collect();
    verses.join("\n")
}
