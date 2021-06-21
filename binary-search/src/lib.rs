use std::cmp::Ordering;

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut b = 0;
    let mut e = array.len();
    while b < e {
        let m = (b + e) / 2;
        match key.cmp(&array[m]) {
            Ordering::Equal => return Some(m),
            Ordering::Less => e = m,
            _ => b = m + 1,
        }
    }
    None
}
