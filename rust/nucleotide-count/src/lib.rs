use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let nucleotides = vec!['A', 'C', 'G', 'T'];
    let mut count = 0;

    if !nucleotides.contains(&nucleotide) {
        return Err(nucleotide);
    }

    for c in dna.chars() {
        if !nucleotides.contains(&c) {
            return Err(c);
        }
        if c == nucleotide {
            count += 1;
        }
    }

    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts: HashMap<char, usize> = [('A', 0), ('C', 0), ('G', 0), ('T', 0)]
        .iter()
        .cloned()
        .collect();

    for c in dna.chars() {
        if !counts.contains_key(&c) {
            return Err(c);
        } else {
            counts.insert(c, counts[&c] + 1);
        }
    }

    Ok(counts)
}
