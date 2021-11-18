#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();

    for i in inputs {
        use CalculatorInput::*;
        match i {
            Add => {
                let rhs = stack.pop()?;
                let lhs = stack.pop()?;
                stack.push(lhs + rhs);
            }
            Subtract => {
                let rhs = stack.pop()?;
                let lhs = stack.pop()?;
                stack.push(lhs - rhs);
            }
            Multiply => {
                let rhs = stack.pop()?;
                let lhs = stack.pop()?;
                stack.push(lhs * rhs);
            }
            Divide => {
                let rhs = stack.pop()?;
                let lhs = stack.pop()?;
                stack.push(lhs / rhs);
            }
            Value(v) => {
                stack.push(*v);
            }
        }
    }

    if stack.len() == 1 {
        stack.pop()
    } else {
        None
    }
}
