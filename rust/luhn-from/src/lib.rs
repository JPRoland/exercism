pub struct Luhn {
    code: Vec<Option<u32>>,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        if self.code.iter().any(|c| c.is_none()) {
            return false;
        }
        if self.code.iter().count() <= 1 {
            return false;
        }

        let total =
            self.code
                .iter()
                .rev()
                .map(|d| d.unwrap())
                .enumerate()
                .fold(0, |acc, (i, d)| {
                    if i % 2 == 0 {
                        acc + d
                    } else if d * 2 > 9 {
                        acc + d * 2 - 9
                    } else {
                        acc + d * 2
                    }
                });
        total % 10 == 0
    }
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<'a> From<&'a str> for Luhn {
    fn from(input: &'a str) -> Self {
        Luhn::from(input.to_string())
    }
}

impl From<String> for Luhn {
    fn from(code: String) -> Self {
        let code = code.replace(" ", "");
        let digits = code
            .chars()
            .map(|c| {
                if let Some(d) = c.to_digit(10) {
                    Some(d)
                } else {
                    None
                }
            })
            .collect();

        Luhn { code: digits }
    }
}

impl From<u8> for Luhn {
    fn from(code: u8) -> Self {
        Luhn::from(code.to_string())
    }
}

impl From<u16> for Luhn {
    fn from(code: u16) -> Self {
        Luhn::from(code.to_string())
    }
}

impl From<u32> for Luhn {
    fn from(code: u32) -> Self {
        Luhn::from(code.to_string())
    }
}

impl From<u64> for Luhn {
    fn from(code: u64) -> Self {
        Luhn::from(code.to_string())
    }
}

impl From<usize> for Luhn {
    fn from(code: usize) -> Self {
        Luhn::from(code.to_string())
    }
}
