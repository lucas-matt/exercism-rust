// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut words:HashMap<&str, u8> = magazine.into_iter().fold(HashMap::new(), |mut map, w| {
        let entry = map.entry(w).or_insert(0);
        *entry += 1;
        map
    });
    for &w in note {
        let entry = words.entry(w).or_insert(0);
        match entry {
            0 => return false,
            _ => *entry -= 1
        }
    }
    true
}
