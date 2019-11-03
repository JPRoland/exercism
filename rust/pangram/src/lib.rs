use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
  let lowercase: HashSet<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
  let sentence_letters: HashSet<char> = sentence
    .chars()
    .map(|c| c.to_ascii_lowercase())
    .filter(|c| c.is_ascii_alphabetic())
    .collect();

  lowercase == sentence_letters
}
