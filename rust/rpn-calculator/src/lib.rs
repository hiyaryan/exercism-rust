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
        return None;
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
                //
                // The reason for using `checked` variants is that the default behavior
                // for integer overflows is to wrap the values, see:
                // https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-overflow
                //
                // Likely that is not a feature of the calculator, so these checked variants
                // will cause the calculation to "fail safely" by returning `None`
                let value = match operation {
                    CalculatorInput::Add => left_operand.checked_add(right_operand),
                    CalculatorInput::Subtract => left_operand.checked_sub(right_operand),
                    CalculatorInput::Multiply => left_operand.checked_mul(right_operand),
                    // checked_div handles the divide-by-zero checks
                    CalculatorInput::Divide => left_operand.checked_div(right_operand),
                    _ => unreachable!(),
                    // Since all the branches use `checked_*` methods which return `Option`,
                    // we can unpack them using one `?`
                }?;

                // Calculate the value to push separately from adding that value back to the
                // stack to avoid repetition
                stack.push(value);
            }
        }
    }

    // the only value remaining on the stack should be the result
    if stack.len() > 1 {
        None
    } else {
        stack.pop()
    }
}
