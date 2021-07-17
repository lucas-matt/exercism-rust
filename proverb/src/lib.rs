pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return "".to_string()
    }
    let firsts = &list[0..list.len()];
    let seconds = &list[1..];
    let pairs = firsts.iter().zip(seconds.iter());
    let mut lines: Vec<String> = pairs.map(|(&first, &second)| {
        format!("For want of a {} the {} was lost.", first, second)
    }).collect();
    lines.push(format!("And all for the want of a {}.", list.first().unwrap()));
    lines.join("\n")
}

