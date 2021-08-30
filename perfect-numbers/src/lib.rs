use std::cmp::Ordering;
use crate::Classification::{Perfect, Abundant, Deficient};

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num < 1 {
        return None;
    }
    let factors = (1..num)
        .filter(|f| num != *f)
        .filter(|f| num % f == 0);
    let aliquot:u64 = factors.sum();
    match num.cmp(&aliquot) {
        Ordering::Equal => Some(Perfect),
        Ordering::Greater => Some(Deficient),
        Ordering::Less => Some(Abundant)
    }
}
