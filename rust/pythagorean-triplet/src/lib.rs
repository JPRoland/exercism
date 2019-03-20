use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut triplets = HashSet::new();
    let mut c: u32;
    for a in 1..(sum / 3) {
        for b in (a + 1)..((sum - 1) / 2) {
            c = sum - a - b;
            if a * a + b * b == c * c {
                triplets.insert([a, b, c]);
                break;
            }
        }
    }

    triplets
}
