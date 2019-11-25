pub fn encrypt(input: &str) -> String {
  let normalized = input
    .to_lowercase()
    .chars()
    .filter(|c| c.is_ascii_alphanumeric())
    .collect::<Vec<char>>();

  if normalized.is_empty() {
    return String::new();
  }

  let row = ((normalized.len() - 1) as f64).sqrt() as usize + 1;
  let col = (0..row)
    .map(|r| {
      (0..)
        .map(move |c| r + c * row)
        .take_while(|&i| i < normalized.len())
        .count()
    })
    .max()
    .unwrap()
    + 1;

  let mut result = (0..row).fold(String::new(), |acc, i| {
    let mut cipher_row = (0..)
      .map(move |c| i + c * row)
      .take_while(|&i| i < normalized.len())
      .fold(String::new(), |mut acc, r| {
        if let Some(c) = normalized.get(r) {
          acc.push(*c)
        }
        acc
      });

    while cipher_row.len() < col {
      cipher_row.push(' ');
    }
    acc + &cipher_row
  });
  result.pop();
  result
}
