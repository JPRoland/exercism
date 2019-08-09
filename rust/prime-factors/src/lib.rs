pub fn factors(mut n: u64) -> Vec<u64> {
    let mut factors = vec![];
    let mut factor = 2;

    while n > 1 {
        if n % factor == 0 {
            factors.push(factor);
            n /= factor;
        } else {
            factor += if factor != 2 { 2 } else { 1 };
        }
    }

    factors
}
