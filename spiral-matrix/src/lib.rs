pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    if size == 0 {
        return Vec::new();
    }
    spiral(1, size)
}

// recursive function that figures out the sprial matrix
fn spiral(seed:u32, size: u32) -> Vec<Vec<u32>> {
    // base case
    if size == 1 {
        return vec!(vec!(seed));
    }

    // build empty matrix
    let mut matrix = vec![vec![0; size as usize]; size as usize];

    // assign outside of matrix
    let outer_indexes = outside_indexes(size);
    let outer_size = outer_indexes.len() as u32;
    let numbers = seed..(seed + outer_size);
    numbers.zip(outer_indexes)
        .for_each(|(num, (x, y))| set(&mut matrix, x, y, num));

    // recurse to build inner matrix, and assign
    if size > 2 {
        let inner = spiral(seed + outer_size, size - 2);
        for y in 0..inner.len() {
            let row = inner.get(y).unwrap();
            for x in 0..row.len() {
                let num = row.get(x).unwrap();
                set(&mut matrix, (x + 1) as u32, (y + 1) as u32, *num);
            }
        }
    }

    matrix
}

fn set(matrix:&mut Vec<Vec<u32>>, x:u32, y:u32, num:u32) {
    let row = matrix.get_mut(y as usize).unwrap();
    row.remove(x as usize);
    row.insert(x as usize, num);
}

fn outside_indexes(size: u32) -> Vec<(u32, u32)> {
    // top
    (0..size-1).map(|x| (x, 0))
        // right
        .chain((0..size-1).map(|y| (size-1, y)))
        // bottom
        .chain((1..size).rev().map(|x| (x, size-1)))
        // left
        .chain((1..size).rev().map(|y| (0, y)))
        .collect()
}