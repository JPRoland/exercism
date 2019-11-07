use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
  words
    .split(|c| char::is_whitespace(c) || c == ',')
    .map(|word| {
      word
        .chars()
        .filter(|&c| c == '\'' || c.is_alphanumeric())
        .collect::<String>()
    })
    .filter(|word| !word.is_empty())
    .fold(HashMap::new(), |mut acc, mut word| {
      if word.starts_with('\'') {
        word = word.replace('\'', "");
      }
      *acc.entry(word.to_lowercase()).or_insert(0) += 1;
      acc
    })
}
