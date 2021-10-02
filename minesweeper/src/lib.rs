
pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let minefield:Vec<Vec<char>> = minefield.into_iter()
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect();

    let mut board:Vec<Vec<char>> = Vec::new();
    for (y, row) in minefield.iter().enumerate() {
        let mut targ_row:Vec<char> = Vec::new();
        for (x, pos) in row.into_iter().enumerate() {
            let square = adj(x as i32, y as i32, &minefield)
                .into_iter()
                .filter(|&c| c == '*')
                .count();
            targ_row.push(match (pos, square) {
                (' ', n) if n > 0 => char::from_digit(n as u32, 10).unwrap(),
                _ => *pos
            });
        }
        board.push(targ_row);
    }

    board.iter()
        .map(|row| row.iter().collect::<String>())
        .collect()
}

fn adj(x:i32, y:i32, minefield:&Vec<Vec<char>>) -> Vec<char> {
    let height = minefield.len() as i32;
    let width = minefield.get(0).unwrap().len() as i32;
    vec!((x-1, y), (x+1, y), (x, y-1), (x, y+1), (x-1, y-1), (x-1, y+1), (x+1, y-1), (x+1, y+1)).into_iter()
        .filter(|&(x, y)| x >= 0 && x < width && y >= 0 && y < height)
        .map(|(x, y)| *minefield.get(y as usize).unwrap().get(x as usize).unwrap())
        .collect()
}