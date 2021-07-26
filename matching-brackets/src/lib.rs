#[macro_use] extern crate maplit;

use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {

    let brackets:HashMap<char, char> = hashmap!{
        '}' => '{',
        ')' => '(',
        ']' => '['
    };
    let opening:Vec<char> = brackets.values().copied().collect();
    let closing:Vec<char> = brackets.keys().copied().collect();

    let mut matches = true;
    let mut stackets:Vec<char> = vec!();

    for char in string.chars() {
        if opening.contains(&char) {
            stackets.push(char);
        }
        if closing.contains(&char) {
            let actual = brackets.get(&char);
            let expected = stackets.pop();
            match (actual, expected) {
                (Some(&x), Some(y)) if x == y => (),
                _ => matches = false
            }
        }
    }

    matches && stackets.is_empty()
}
