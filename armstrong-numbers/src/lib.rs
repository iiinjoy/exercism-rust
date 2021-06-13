fn num_of_digits(num: u32) -> u32 {
    let one_digit = 1;
    for i in (1..=9).rev() {
        if num / 10u32.pow(i) > 0 {
            return one_digit + i;
        }
    }
    one_digit
}

pub fn is_armstrong_number(num: u32) -> bool {
    let num_of_digits = num_of_digits(num);
    let mut rem = num;
    let mut sum = 0;
    for i in (0..num_of_digits).rev() {
        let d = rem / 10u32.pow(i);
        sum += d.pow(num_of_digits);
        rem %= 10u32.pow(i);
    }
    sum == num
}

#[test]
fn test_num_of_digits() {
    assert_eq!(num_of_digits(0), 1);
    assert_eq!(num_of_digits(1), 1);
    assert_eq!(num_of_digits(10), 2);
    assert_eq!(num_of_digits(2000), 4);
    assert_eq!(num_of_digits(30_000), 5);
}
