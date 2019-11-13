/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
  let code = code.replace(" ", "");
  if code.chars().any(|c| !c.is_digit(10)) {
    return false;
  }
  if code.chars().count() <= 1 {
    return false;
  }
  let total = code
    .chars()
    .rev()
    .map(|c| c.to_digit(10).unwrap())
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
