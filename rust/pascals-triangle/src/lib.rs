pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut triangle = vec![];
        for row in 0..self.row_count {
            triangle.push(PascalsTriangle::row(row));
        }

        triangle
    }

    fn row(row_number: u32) -> Vec<u32> {
        let mut row = vec![1];

        for n in 0..row_number {
            row.push(row[n as usize] * (row_number - n) / (n + 1));
        }
        row
    }
}
