use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
  let mut seen: HashSet<char> = HashSet::new();
  // for c in candidate.to_lowercase().chars() {
  //   if !c.is_alphabetic() {
  //     continue;
  //   }
  //   if seen.contains(&c) {
  //     return false;
  //   }
  //   seen.insert(c);
  // }
  // true
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
