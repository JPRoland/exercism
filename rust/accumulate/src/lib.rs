pub fn map<T, S>(input: Vec<T>, mut function: impl FnMut(T) -> S) -> Vec<S> {
  let mut result = Vec::new();

  for item in input {
    result.push(function(item));
  }

  result
}
