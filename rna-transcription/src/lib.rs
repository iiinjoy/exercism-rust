#[derive(Debug, PartialEq)]
pub struct Dna(Vec<char>);

#[derive(Debug, PartialEq)]
pub struct Rna(Vec<char>);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let strand = dna
            .chars()
            .enumerate()
            .try_fold(Vec::new(), |mut acc, (i, c)| match c {
                'A' | 'C' | 'G' | 'T' => {
                    acc.push(c);
                    Ok(acc)
                }
                _ => Err(i),
            })?;
        Ok(Dna(strand))
    }

    pub fn into_rna(self) -> Rna {
        Rna(self
            .0
            .iter()
            .map(|&c| match c {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => c,
            })
            .collect())
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let strand = rna
            .chars()
            .enumerate()
            .try_fold(Vec::new(), |mut acc, (i, c)| match c {
                'A' | 'C' | 'G' | 'U' => {
                    acc.push(c);
                    Ok(acc)
                }
                _ => Err(i),
            })?;
        Ok(Rna(strand))
    }
}
