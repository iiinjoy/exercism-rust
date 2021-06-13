pub fn reply(message: &str) -> &str {
    let trimmed = message.trim();
    let is_question = trimmed.ends_with('?');
    let is_yell = trimmed.find(|c: char| c.is_uppercase()).is_some()
        && trimmed.to_string() == trimmed.to_uppercase();
    let is_empty = trimmed.is_empty();
    match (is_empty, is_question, is_yell) {
        (true, _, _) => "Fine. Be that way!",
        (_, true, false) => "Sure.",
        (_, false, true) => "Whoa, chill out!",
        (_, true, true) => "Calm down, I know what I'm doing!",
        _ => "Whatever.",
    }
}
