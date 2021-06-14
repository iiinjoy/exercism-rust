pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split_whitespace()
        .flat_map(|s| s.split('-'))
        .map(|s| {
            s.chars()
                .filter(|c| !c.is_ascii_punctuation())
                .collect::<String>()
        })
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.chars()
                .fold(
                    (String::from(&s[0..1]), true),
                    |(mut s, is_prev_upper), c| {
                        let is_upper = c.is_ascii_uppercase();
                        if !is_prev_upper && is_upper {
                            s.push(c);
                            (s, is_upper)
                        } else {
                            (s, is_upper)
                        }
                    },
                )
                .0
        })
        .map(|c| c.to_ascii_uppercase())
        .collect()
}
