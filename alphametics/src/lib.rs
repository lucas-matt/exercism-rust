use std::collections::{HashMap, HashSet};

struct Solver {
    equation: Equation,
    leaders: HashSet<char>,
    variables: HashSet<char>
}

struct Equation(Vec<String>, String);

impl Equation {

    pub fn satisfied(&self) -> bool {
        let left:i64 = (&self.0).into_iter()
            .map(|opand| opand.parse::<i64>().unwrap())
            .sum();
        let right = self.1.parse().unwrap();
        left == right
    }

    pub fn partial(&self) -> bool {
        // e.g. 7AB + 5ABC would become the min: 700 + 5000
        let min_left:i64 = (&self.0).into_iter()
            .map(|opand| (opand.chars().next().unwrap(), opand.len()))
            .filter(|(head, _) | head.is_numeric())
            .map(|(head, size)| {
                let x = head.to_string().parse::<i64>().unwrap();
                let scale = 10_i64.pow((size - 1) as u32);
                x * scale
            })
            .sum();
        // e.g. 7ABC would become 8000
        let max_right = {
            let head = match (&self.1).chars().next().unwrap() {
                num if num.is_numeric() => num.to_string().parse().unwrap(),
                _ => 9
            };
            (head + 1) * 10_i64.pow((&self.1.len() - 1) as u32)
        };
        // if the minimum left is already bigger than the maximum right then we cannot ever balance
        min_left < max_right
    }

}

impl Solver {
    pub fn new(equation: &str) -> Self {
        Solver {
            equation: unpack(equation),
            variables: variables(equation),
            leaders: leaders(equation)
        }
    }

    pub fn solve(&self) -> Option<HashMap<char, u8>> {
        let variables:Vec<char> = self.variables.clone().into_iter().collect();
        return self.try_solve(&self.equation, &variables, &vec!());
    }

    fn try_solve(&self, equation: &Equation, variables: &Vec<char>, values: &Vec<u8>) -> Option<HashMap<char, u8>> {
        if variables.is_empty() {
            if equation.satisfied() {
                return Some(HashMap::new());
            }
            return None;
        }

        // give up early if we've gone down a blind alley
        if !equation.partial() {
            return None;
        }

        let (next, rest) = {
            let mut variables = variables.clone();
            (variables.pop().unwrap(), variables)
        };

        for num in 0..10 {
            // skip values already taken or zero if a leader
            if values.contains(&num) || (num == 0 && self.leaders.contains(&next)) {
                continue;
            }
            let mut values = values.clone();
            values.push(num);
            let equation = sub_eqn(equation, next.to_string(), num);
            let solution = self.try_solve(&equation, &rest, &values);

            // unwind result on success
            match solution {
                Some(mut map) => {
                    map.insert(next, num);
                    return Some(map);
                },
                _ => ()
            }

        }

        None
    }
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let solver = Solver::new(input);
    solver.solve()
}

fn sub_eqn(equation: &Equation, variable:String, value:u8) -> Equation {
    let opands:Vec<String> = equation.0.iter()
        .map(|opand| sub_one(opand.clone(), &variable, &value))
        .collect();

    let result = sub_one(equation.1.clone(), &variable, &value);
    Equation(opands, result)
}

fn sub_one(opand: String, variable:&String, value:&u8) -> String {
    opand.replace(variable, value.to_string().as_str())
}

fn unpack(equation:&str) -> Equation {
    let parts:Vec<&str> = equation.split(" == ").collect();
    let left:Vec<String> = parts[0].split("+")
        .map(|opand| opand.trim().to_string())
        .collect();
    let right:String = parts[1].trim().to_string();
    Equation(left, right)
}

fn leaders(equation: &str) -> HashSet<char> {
    equation.split(" ")
        .map(|chunk| chunk.trim())
        .filter(|chunk| chunk.chars().all(|c| c.is_alphabetic()))
        .map(|chunk| chunk.chars().next().unwrap())
        .collect::<HashSet<char>>()
}

fn variables(equation: &str) -> HashSet<char> {
    equation.chars()
        .filter(|c| c.is_alphabetic())
        .collect()
}
