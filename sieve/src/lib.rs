pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut prime: Vec<bool> = (0..upper_bound - 1).map(|_| true).collect();
    for i in 0..prime.len() {
        if prime[i] {
            let num = i + 2;
            let mut mult_idx = i + num;
            while mult_idx < prime.len() {
                prime[mult_idx] = false;
                mult_idx += num;
            }
        }
    }
    prime
        .iter()
        .enumerate()
        .filter(|(_, &v)| v)
        .map(|(i, _)| i as u64 + 2)
        .collect()
}
