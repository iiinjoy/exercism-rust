pub fn factors(n: u64) -> Vec<u64> {
    match n {
        0..=1 => vec![],
        _ => {
            let mut factors = vec![];
            let mut rem = n;
            (2..=n).try_for_each(|n| {
                if rem == 1 {
                    None
                } else {
                    while rem % n == 0 {
                        factors.push(n);
                        rem /= n;
                    }
                    Some(())
                }
            });
            factors
        }
    }
}
