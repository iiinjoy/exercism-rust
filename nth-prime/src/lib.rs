pub fn nth(n: u32) -> u32 {
    let is_prime = |x| {
        for i in 2..x - 1 {
            if x % i == 0 {
                return false;
            }
        }
        true
    };

    (2..).filter(|x| is_prime(*x)).nth(n as usize).unwrap()
}
