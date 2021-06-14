pub struct PascalsTriangle {
    triangle: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut triangle: Vec<Vec<u32>> = vec![];
        for r in 0..row_count {
            let row: Vec<u32> = match r {
                0 => vec![1],
                1 => vec![1, 1],
                _ => {
                    let prev = triangle.last().unwrap();
                    let mut row = prev.windows(2).fold(vec![1], |mut r, wnd| {
                        r.push(wnd[0] + wnd[1]);
                        r
                    });
                    row.push(1);
                    row
                }
            };
            triangle.push(row);
        }
        Self { triangle }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.triangle.clone()
    }
}
