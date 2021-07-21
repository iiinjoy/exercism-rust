use itertools::Itertools;
use std::collections::HashMap;

fn to_number(token: &str, table: &HashMap<char, u8>) -> Option<u64> {
    let alphadigit = token.chars().next()?;
    let digit = table.get(&alphadigit)?;
    if token.len() > 1 && *digit == 0 {
        return None;
    }
    token.chars().try_fold(0u64, |acc, c| {
        let digit = table.get(&c)?;
        Some(acc * 10 + *digit as u64)
    })
}

fn sum_expr(expr: &[&str], table: &HashMap<char, u8>) -> Option<u64> {
    expr.iter().try_fold(0u64, |acc, tok| {
        let term = to_number(tok, table)?;
        Some(acc + term)
    })
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut table = HashMap::new();
    let mut left_expr = Vec::new();
    let mut right_expr = Vec::new();
    let mut is_right_expr = false;
    input.split(' ').for_each(|tok| match tok {
        "==" => is_right_expr = true,
        "+" => (),
        _ => {
            tok.chars().for_each(|c| {
                table.entry(c).or_insert(0u8);
            });
            if is_right_expr {
                right_expr.push(tok);
            } else {
                left_expr.push(tok);
            }
        }
    });
    let mut solutions = Vec::new();

    let keys: Vec<char> = table.keys().cloned().collect();
    (0..=9).permutations(table.len()).for_each(|p| {
        let t: HashMap<char, u8> = keys.iter().cloned().zip(p.iter().cloned()).collect();
        let lsum_opt = sum_expr(&left_expr, &t);
        let rsum_opt = sum_expr(&right_expr, &t);
        match (lsum_opt, rsum_opt) {
            (Some(lsum), Some(rsum)) if lsum == rsum => solutions.push(t),
            _ => (),
        }
    });
    println!("{:?}", solutions);
    if solutions.len() == 1 {
        solutions.pop()
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_number_empty() {
        let table = HashMap::new();
        assert_eq!(to_number("", &table), None);
    }

    #[test]
    fn test_to_number_zero_is_ok() {
        let table: HashMap<char, u8> = [('A', 0)].iter().cloned().collect();
        assert_eq!(to_number("A", &table), Some(0));
    }

    #[test]
    fn test_to_number_leading_zero_is_not_valid() {
        let table: HashMap<char, u8> = [('A', 0), ('B', 1), ('C', 2)].iter().cloned().collect();
        assert_eq!(to_number("ABC", &table), None);
    }

    #[test]
    fn test_to_number_multidigit_correct() {
        let table: HashMap<char, u8> = [('A', 1), ('B', 2), ('C', 3)].iter().cloned().collect();
        assert_eq!(to_number("BAC", &table), Some(213));
    }

    #[test]
    fn test_to_number_multidigit_invalid_table() {
        let table: HashMap<char, u8> = [('A', 1), ('C', 3)].iter().cloned().collect();
        assert_eq!(to_number("ABC", &table), None);
    }
}
