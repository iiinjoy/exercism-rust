// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

fn parse_one_digit(grid: &str) -> &'static str {
    match grid {
        #[rustfmt::skip]
        concat!(" _ ",
                "| |",
                "|_|",
                "   ") => "0",
        #[rustfmt::skip]
        concat!("   ",
                "  |",
                "  |",
                "   ") => "1",
        #[rustfmt::skip]
        concat!(" _ ",
                " _|",
                "|_ ",
                "   ") => "2",
        #[rustfmt::skip]
        concat!(" _ ",
                " _|",
                " _|",
                "   ") => "3",
        #[rustfmt::skip]
        concat!("   ",
                "|_|",
                "  |",
                "   ") => "4",
        #[rustfmt::skip]
        concat!(" _ ",
                "|_ ",
                " _|",
                "   ") => "5",
        #[rustfmt::skip]
        concat!(" _ ",
                "|_ ",
                "|_|",
                "   ") => "6",
        #[rustfmt::skip]
        concat!(" _ ",
                "  |",
                "  |",
                "   ") => "7",
        #[rustfmt::skip]
        concat!(" _ ",
                "|_|",
                "|_|",
                "   ") => "8",
        #[rustfmt::skip]
        concat!(" _ ",
                "|_|",
                " _|",
                "   ") => "9",
        _ => "?",
    }
}

fn string_to_chunks(s: String, chunk_size: usize) -> Vec<String> {
    s.chars()
        .collect::<Vec<char>>()
        .chunks_exact(chunk_size)
        .map(|c| c.iter().collect::<String>())
        .collect()
}

pub fn convert(input: &str) -> Result<String, Error> {
    let rows = input.split('\n').map(|r| r.into()).collect::<Vec<String>>();
    if rows.len() % 4 != 0 {
        return Err(Error::InvalidRowCount(rows.len()));
    }
    for r in &rows {
        if r.len() % 3 != 0 {
            return Err(Error::InvalidColumnCount(r.len()));
        }
    }
    Ok(rows
        .as_slice()
        .chunks_exact(4)
        .map(|r0123| {
            let r0 = string_to_chunks(r0123[0].clone(), 3);
            let r1 = r0.iter();
            let r2 = r1.zip(string_to_chunks(r0123[1].clone(), 3));
            let r3 = r2.zip(string_to_chunks(r0123[2].clone(), 3));
            let r4 = r3.zip(string_to_chunks(r0123[3].clone(), 3));
            r4.map(|(((c1, c2), c3), c4)| c1.clone() + &c2 + &c3 + &c4)
                .map(|s| parse_one_digit(&s))
                .collect::<Vec<_>>()
                .join("")
        })
        .collect::<Vec<_>>()
        .join(","))
}
