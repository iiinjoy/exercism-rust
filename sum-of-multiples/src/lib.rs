pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let factors = factors.iter().filter(|f| **f > 0).collect::<Vec<_>>();
    (1..limit)
        .filter(|x| factors.iter().any(|f| x % *f == 0))
        .sum()
}
