pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_points:Vec<(usize, usize)> = Vec::new();
    for r in 0..input.len() {
        let row = &input[r];
        for c in 0..row.len() {
            let col: Vec<u64> = input.iter().map(|row| row[c]).collect();
            let point = row[c];
            if row.into_iter().all(|&r| point >= r) && col.into_iter().all(|c| point <= c) {
                saddle_points.push((r, c))
            }
        }
    }
    saddle_points
}
