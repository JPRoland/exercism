pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
  if upper_bound < 2 {
    return vec![];
  }
  let mut primes = vec![true; upper_bound as usize + 1];
  primes[0] = false;
  if upper_bound >= 1 {
    primes[1] = false;
  }

  for n in 2..=(upper_bound as f64).sqrt() as usize {
    if primes[n as usize] {
      let mut square = n * n;
      while square <= upper_bound as usize {
        primes[square as usize] = false;
        square += n;
      }
    }
  }

  primes
    .iter()
    .enumerate()
    .filter_map(|(p, &is_p)| if is_p { Some(p as u64) } else { None })
    .collect()
}
