use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
  Abundant,
  Perfect,
  Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
  if num == 0 {
    return None;
  }

  match aliquot_sum(num).cmp(&num) {
    Ordering::Equal => Some(Classification::Perfect),
    Ordering::Greater => Some(Classification::Abundant),
    Ordering::Less => Some(Classification::Deficient),
  }
}

fn aliquot_sum(num: u64) -> u64 {
  let square_root = (num as f64).sqrt() as u64;

  let mut sum = (2..=square_root).fold(
    1,
    |acc, n| if num % n == 0 { acc + n + num / n } else { acc },
  );

  if square_root * square_root == num {
    sum -= square_root;
  }

  sum
}
