#[derive(Debug, PartialEq)]
pub struct DNA {
    nucleotides: String,
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    nucleotides: String,
}

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        match collect_valid_nucleotides(dna, "ACGT") {
            Ok(nucleotides) => Ok(DNA { nucleotides }),
            Err(i) => Err(i),
        }
    }

    pub fn into_rna(self) -> RNA {
        RNA::new(
            &self
                .nucleotides
                .chars()
                .map(|nuc| match nuc {
                    'A' => 'U',
                    'C' => 'G',
                    'G' => 'C',
                    'T' => 'A',
                    _ => nuc,
                })
                .collect::<String>(),
        )
        .unwrap()
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        match collect_valid_nucleotides(rna, "ACGU") {
            Ok(nucleotides) => Ok(RNA { nucleotides }),
            Err(i) => Err(i),
        }
    }
}

fn collect_valid_nucleotides(
    nucleotide_string: &str,
    valid_nucleotides: &str,
) -> Result<String, usize> {
    let result: String = nucleotide_string
        .chars()
        .take_while(|&c| valid_nucleotides.contains(c))
        .collect();

    if result.len() == nucleotide_string.len() {
        Ok(result)
    } else {
        Err(result.len())
    }
}
