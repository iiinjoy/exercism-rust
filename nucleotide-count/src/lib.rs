use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !"ACGT".contains(nucleotide) {
        Err(nucleotide)
    } else {
        dna.chars().try_fold(0, |acc, c| match c {
            'A' | 'C' | 'G' | 'T' => {
                if c == nucleotide {
                    Ok(acc + 1)
                } else {
                    Ok(acc)
                }
            }
            _ => Err(c),
        })
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let counters: HashMap<char, usize> = [('A', 0), ('C', 0), ('G', 0), ('T', 0)]
        .iter()
        .cloned()
        .collect();
    dna.chars().try_fold(counters, |mut acc, c| match c {
        'A' | 'C' | 'G' | 'T' => {
            let count = acc.entry(c).or_insert(0);
            *count += 1;
            Ok(acc)
        }
        _ => Err(c),
    })
}
