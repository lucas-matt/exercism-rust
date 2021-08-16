pub struct Triangle(u64, u64, u64);

fn valid(x:u64, y:u64, z:u64) -> bool {
    let size = [x, y, z].iter().all(|&n| n > 0);
    let scale = ![(x, y, z), (y, z, x), (z, x, y)].iter()
        .any(|(x, y, z)| x > &(y + z));
    size && scale
}

impl Triangle {

    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        match sides {
            [x, y, z] if valid(x, y, z) => Some(Triangle(x, y, z)),
            _ => None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.pairs().iter().all(|(x, y)| x == y)
    }

    pub fn is_scalene(&self) -> bool {
        self.pairs().iter().all(|(x, y)| x != y)
    }

    pub fn is_isosceles(&self) -> bool {
        self.pairs().iter().filter(|(x, y)| x == y).count() == 1
    }

    fn pairs(&self) -> Vec<(u64, u64)> {
        [(self.0, self.1), (self.1, self.2), (self.2, self.0)]
            .iter()
            .map(|t| t.clone())
            .collect::<Vec<(u64, u64)>>()
    }

}
