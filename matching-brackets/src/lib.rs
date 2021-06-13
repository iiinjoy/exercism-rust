pub fn brackets_are_balanced(string: &str) -> bool {
    fn close_bracket(c: char) -> char {
        match c {
            '[' => ']',
            '(' => ')',
            '{' => '}',
            _ => c,
        }
    }

    let mut brackets = Vec::new();
    string
        .chars()
        .try_for_each(|c| match c {
            '[' | '(' | '{' => {
                brackets.push(c);
                Some(())
            }
            ']' | ')' | '}' => {
                if brackets.last().copied().map(close_bracket) == Some(c) {
                    brackets.pop();
                    Some(())
                } else {
                    None
                }
            }
            _ => Some(()),
        })
        .is_some()
        && brackets.is_empty()
}
