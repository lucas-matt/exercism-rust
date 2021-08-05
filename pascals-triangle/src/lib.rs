pub struct PascalsTriangle {
    size: u32
}

impl PascalsTriangle {

    pub fn new(row_count: u32) -> Self {
        PascalsTriangle {
            size: row_count
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.row(self.size)
    }

    fn row(&self, n:u32) -> Vec<Vec<u32>> {
        match n {
            0 => vec!(),
            1 => vec!(vec!(1)),
            _ => {
                let mut tri = self.row(n - 1);
                let prev= tri.last().unwrap();
                let middle:Vec<u32> = prev.windows(2)
                    .map(|slice| slice.into_iter().sum())
                    .collect();
                let row:Vec<u32> = vec!(1).into_iter()
                    .chain(middle)
                    .chain(vec!(1))
                    .collect();
                tri.push(row);
                tri
            }
        }
    }

}
