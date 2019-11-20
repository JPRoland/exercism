use rand::{thread_rng, Rng};

pub fn private_key(p: u64) -> u64 {
    thread_rng().gen_range(2, p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    let mut result: u64 = 1;
    let mut g = g % p;
    let mut a = a;
    while a > 0 {
        if a % 2 == 1 {
            result = (result * g) % p;
        }
        a >>= 1;
        g = (g * g) % p;
    }
    result
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    public_key(p, b_pub, a)
}
