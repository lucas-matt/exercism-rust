use crate::CalculatorInput::{Value, Add, Subtract, Multiply, Divide};

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
        sum = match op {
            Some((i, _)) if i >= 2 => {
                let head = &sum[..i-2];
                let tail = &sum[i+1..];
                head.iter()
                    .chain(eval(&sum[i-2], &sum[i-1], &sum[i]).iter())
                    .chain(tail.iter())
                    .map(|c| *c)
                    .collect()
            },
            _ => break
        }
    }
    match sum.as_slice() {
        &[Value(v)] => Some(v),
        _ => None
    }
}

fn eval(x:&CalculatorInput, y:&CalculatorInput, func:&CalculatorInput) -> Vec<CalculatorInput> {
    let result = match (x, y, func){
        (Value(x), Value(y), func) => {
            match func {
                Add => x + y,
                Subtract => x - y,
                Multiply => x * y,
                Divide => x / y,
                _ => panic!("Unsupported")
            }
        },
        _ => panic!("Unsupported")
    };
    vec!(Value(result))
}