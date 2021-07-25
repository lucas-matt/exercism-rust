pub fn reply(message: &str) -> &str {

    let message = message.trim();

    let has_alphabetic = message.chars().any(|c| c.is_alphabetic());
    let is_empty = message.is_empty();
    let is_shouting = message.chars()
        .filter(|c| c.is_alphabetic())
        .all(|c| c.is_uppercase()) & has_alphabetic;
    let is_question = message.ends_with("?");

    match (is_empty, is_question, is_shouting) {
        (true, _, _) => "Fine. Be that way!",
        (_, true, false) => "Sure.",
        (_, true, true) => "Calm down, I know what I'm doing!",
        (_, false, true) => "Whoa, chill out!",
        _ => "Whatever."
    }

}
