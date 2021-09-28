use crate::Comparison::{Unequal, Equal, Sublist, Superlist};

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    match (_first_list, _second_list) {
        (x, y) if x == y => Equal,
        (x, y) if is_sub(x, y) => Sublist,
        (x, y) if is_sub(y, x) => Superlist,
        _ => Unequal
    }
}

fn is_sub<T: PartialEq>(x:&[T], y:&[T]) -> bool {
    if x.is_empty() {
        return true
    }
    y.windows(x.len()).any(|w| x == w)
}
