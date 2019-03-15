pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit).fold(0, |acc, x| {
        if !factors.iter().any(|&n| n != 0 && x % n == 0) {
            return acc;
        }
        acc + x
    })
}
