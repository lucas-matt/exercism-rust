#[derive(Debug)]
pub struct HighScores<'h>{
    scores: &'h[u32]
}

impl<'h> HighScores<'h> {

    pub fn new(scores: &'h[u32]) -> Self {
        HighScores {
            scores
        }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut scores = self.scores.to_vec();
        scores.sort();
        let top_three = scores.iter().rev().take(3);
        top_three.copied().collect()
    }

}
