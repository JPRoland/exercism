pub fn raindrops(n: u32) -> String {
    let factors = [(3, "Pling"), (5, "Plang"), (7, "Plong")];
    let mut result = String::new();

    for (f, s) in &factors {
        if n % f == 0 {
            result.push_str(s);
        }
    }

    if result.is_empty() {
        return n.to_string();
    }

    result
}
