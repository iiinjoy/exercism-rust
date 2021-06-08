pub fn build_proverb(list: &[&str]) -> String {
    list.windows(2)
        .map(|s| format!("For want of a {} the {} was lost.\n", s[0], s[1]))
        .collect::<Vec<String>>()
        .join("")
        + match list {
            &[s, ..] => format!("And all for the want of a {}.", s),
            _ => String::from(""),
        }
        .as_str()
}
