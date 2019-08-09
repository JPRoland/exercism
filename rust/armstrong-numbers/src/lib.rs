pub fn is_armstrong_number(num: u32) -> bool {
    let digits = number_to_vec(num);
    num == digits
        .iter()
        .fold(0, |acc, x| acc + x.pow(digits.len() as u32))
}

fn number_to_vec(n: u32) -> Vec<u32> {
    let mut digits = vec![];
    let mut n = n;
    while n > 9 {
        digits.push(n % 10);
        n /= 10;
    }
    digits.push(n);
    digits.reverse();
    digits
}
