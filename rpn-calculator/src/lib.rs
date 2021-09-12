use crate::CalculatorInput::{Value, Add, Subtract, Multiply, Divide};
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut sum = inputs.to_vec();
    while sum.len() > 1 {
        let op = sum.iter().enumerate().find(|(_, op)| *&[Add, Subtract, Multiply, Divide].contains(op));
        match op {
            None => break,
            Some((i, op)) => {
                if let &[Value(x), Value(y), action] = sum.drain(i-2..=i).as_slice() {
                    let value = match action {
                        Add => Value(x+y),
                        Subtract => Value(x-y),
                        Multiply => Value(x*y),
                        Divide => Value(x/y),
                        _ => panic!("Unsupported action")
                    };
                    sum.insert(i-2, value);
                };
            }
        }
    }
    match sum.as_slice() {
        &[Value(v)] => Some(v),
        _ => None
    }
}
