use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
  h.iter().fold(BTreeMap::new(), |mut acc, (score, letters)| {
    for c in letters {
      acc.insert(c.to_lowercase().collect::<Vec<_>>()[0], *score);
    }
    acc
  })
}
