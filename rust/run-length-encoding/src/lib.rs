pub fn encode(source: &str) -> String {
  let sentinel_string = format!("{}{}", source, "\0");
  let mut char_count = 0;
  sentinel_string
    .chars()
    .zip(sentinel_string.chars().skip(1))
    .fold(String::new(), |mut acc, (ch, prev)| {
      char_count += 1;
      if ch == '\0' || ch != prev {
        if char_count <= 1 {
          acc.push(ch);
        } else {
          acc += &format!("{}{}", char_count, ch);
        }
        char_count = 0;
      }
      acc
    })
}

pub fn decode(source: &str) -> String {
  let mut digits = String::new();
  source.chars().fold(String::new(), |mut acc, ch| {
    if ch.is_digit(10) {
      digits.push(ch);
    } else {
      match digits.parse() {
        Ok(n) => acc.push_str(&ch.to_string().repeat(n)),
        Err(_) => acc.push(ch),
      }
      digits.clear();
    }
    acc
  })
}
