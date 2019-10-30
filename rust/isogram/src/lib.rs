use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
  let mut seen: HashSet<char> = HashSet::new();
  candidate.to_lowercase().chars().all(|c| {
    if !c.is_alphabetic() {
      true
    } else if seen.contains(&c) {
      false
    } else {
      seen.insert(c);
      true
    }
  })
}
