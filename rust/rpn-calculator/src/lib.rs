#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    // ensure there are CalculatorInput to perform operations in RPN
    if inputs.is_empty() {
        return None
    }

    // a stack to hold operands
    let mut stack = Vec::new();

    // loop through all of the CalculatorInput
    for input in inputs {
        match input {
            CalculatorInput::Value(i) => stack.push(*i),
            operation => {
                // before matching the operation ensure operands are valid

                // pop the right operand from the stack
                let right_operand = stack.pop()?;

                // pop the left operand from the stack
                let left_operand = stack.pop()?;

                // perform operation then push it onto the stack
                match operation {
                    CalculatorInput::Add => stack.push(left_operand + right_operand),
                    CalculatorInput::Subtract => stack.push(left_operand - right_operand),
                    CalculatorInput::Multiply => stack.push(left_operand * right_operand),
                    CalculatorInput::Divide => {
                        // if the divisor is 0 save the world from ending
                        if right_operand == 0 {
                            return None
                        }

                        stack.push(left_operand / right_operand)
                    }
                    _ => (),
                }
            },
        }
    }

    // the only value remaining on the stack should be the result
    if stack.len() > 1 {
        None
    } else {
        stack.pop()
    }
}