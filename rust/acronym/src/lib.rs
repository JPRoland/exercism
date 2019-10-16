pub fn abbreviate(phrase: &str) -> String {
  // phrase.split_whitespace().fold(String::new(), |mut acc, s| {
  //   acc.push(s.chars().next().unwrap());
  //   acc
  // })
  phrase
    .split_whitespace()
    .flat_map(|word| {
      word.split('-').map(move |split_word: &str| {
        if split_word
          .chars()
          .all(|c| !c.is_alphabetic() || c.is_uppercase())
        {
          word.chars().nth(0).unwrap().to_string()
        } else {
          split_word
            .chars()
            .enumerate()
            .filter_map(|(i, c)| {
              if i == 0 || c.is_uppercase() {
                Some(c.to_uppercase().nth(0).unwrap())
              } else {
                None
              }
            })
            .collect::<String>()
        }
      })
    })
    .collect()
}
