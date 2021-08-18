#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if [rank, file].iter().any(|&i| i < 0 || i >= 8) {
            return None
        }
        Some(
            ChessPosition {
                rank, file
            }
        )
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen {
            position
        }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let (ax, ay, bx, by) = (self.position.file, self.position.rank, other.position.file, other.position.rank);
        let dx = (ax - bx).abs();
        let dy = (ay - by).abs();
        dx == 0 || dy == 0 || dx == dy
    }
}
