pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    input
        .iter()
        .enumerate()
        .fold(Vec::new(), |mut acc, (i, row)| {
            for (j, &x) in row.iter().enumerate() {
                let horizontal = row.iter().copied();
                let vertical = input.iter().map(|row| row[j]);

                if x == vertical.min().unwrap() && x == horizontal.max().unwrap() {
                    acc.push((i, j));
                }
            }
            acc
        })
}
