use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq)]
pub struct Palindrome {
    factors: Vec<(u64, u64)>,
}

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Palindrome {
        Palindrome {
            factors: vec![(a, b)],
        }
    }

    pub fn value(&self) -> u64 {
        let mut value = 0;
        if let Some(first) = self.factors.first() {
            value = first.0 * first.1;
        }
        value
    }

    pub fn insert(&mut self, a: u64, b: u64) {
        self.factors.push((a, b));
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut palindromes: BTreeMap<u64, Palindrome> = BTreeMap::new();
    for i in min..=max {
        for j in i..=max {
            let product = i * j;
            if is_palindrome(product) {
                palindromes
                    .entry(product)
                    .and_modify(|p| p.insert(i, j))
                    .or_insert_with(|| Palindrome::new(i, j));
            }
        }
    }

    if palindromes.is_empty() {
        return None;
    }

    let min = *palindromes.iter().nth(0).unwrap().0;
    let max = *palindromes.iter().nth_back(0).unwrap().0;

    let smallest = palindromes.remove(&min);
    let largest = palindromes.remove(&max);

    match (smallest, largest) {
        (Some(smallest), Some(largest)) => Some((smallest, largest)),
        _ => None,
    }
}

fn is_palindrome(n: u64) -> bool {
    let digits = format!("{}", n);
    digits.chars().eq(digits.chars().rev())
}
