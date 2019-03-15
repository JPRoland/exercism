pub fn nth(n: u32) -> u32 {
    let mut count = 0;
    let mut i = 1;

    loop {
        i += 1;
        if is_prime(i) {
            count += 1;
        }
        if count == n + 1 {
            break i;
        }
    }
}

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }

    let mut i = 2;

    loop {
        if i * i > n {
            break;
        }

        if n % i == 0 || i * i == n {
            return false;
        }

        i += 1;
    }

    true
}
