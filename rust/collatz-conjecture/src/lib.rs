pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    let mut cur = n;
    let mut steps = 0;
    while cur != 1 {
        if cur % 2 == 0 {
            cur /= 2;
        } else {
            cur = cur * 3 + 1;
        }
        steps += 1;
    }
    Some(steps)
}
