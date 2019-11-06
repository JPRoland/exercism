#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base <= 1 {
        return Err(Error::InvalidInputBase);
    } else if to_base <= 1 {
        return Err(Error::InvalidOutputBase);
    }

    let from_num = convert_from(from_base, &number)?;

    Ok(convert_to(to_base, from_num))
}

fn convert_from(base: u32, digits: &[u32]) -> Result<u32, Error> {
    digits.iter().try_fold(0, |acc, d| {
        if d < &base {
            Ok(acc * base + d)
        } else {
            Err(Error::InvalidDigit(*d))
        }
    })
}

fn convert_to(base: u32, mut num: u32) -> Vec<u32> {
    let mut result = Vec::new();

    while num > 0 {
        result.insert(0, num % base);
        num /= base;
    }

    result
}
