#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn calc(a: i32, b: i32, calculator: &CalculatorInput) -> i32 {
    return match calculator {
        CalculatorInput::Add => a + b,
        CalculatorInput::Divide => a / b,
        CalculatorInput::Subtract => a - b,
        CalculatorInput::Multiply => a * b,
        _ => 0,
    };
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    if inputs.len() == 0 {
        return None;
    }
    let mut value_stack: Vec<i32> = Vec::new();
    for input in inputs {
        match input {
            CalculatorInput::Value(v) => value_stack.push(*v),
            _ => {
                if value_stack.len() < 2 {
                    return None;
                }
                let b = value_stack.pop()?;
                let a = value_stack.pop()?;
                value_stack.push(calc(a, b, input));
            }
        }
    }
    if value_stack.len() == 1 {
        return Some(value_stack.pop()?);
    }
    return None;
}

#[cfg(test)]
fn calculator_input(s: &str) -> Vec<CalculatorInput> {
    s.split_whitespace()
        .map(|s| match s {
            "+" => CalculatorInput::Add,
            "-" => CalculatorInput::Subtract,
            "*" => CalculatorInput::Multiply,
            "/" => CalculatorInput::Divide,
            n => CalculatorInput::Value(n.parse().unwrap()),
        })
        .collect()
}

#[test]
fn test_empty_input_returns_none() {
    let input = calculator_input("");
    assert_eq!(evaluate(&input), None);
}

#[test]
fn test_simple_value() {
    let input = calculator_input("10");
    assert_eq!(evaluate(&input), Some(10));
}

#[test]
fn test_simple_addition() {
    let input = calculator_input("2 2 +");
    assert_eq!(evaluate(&input), Some(4));
}

#[test]
fn test_simple_subtraction() {
    let input = calculator_input("7 11 -");
    assert_eq!(evaluate(&input), Some(-4));
}

#[test]
fn test_simple_multiplication() {
    let input = calculator_input("6 9 *");
    assert_eq!(evaluate(&input), Some(54));
}

#[test]
fn test_simple_division() {
    let input = calculator_input("57 19 /");
    assert_eq!(evaluate(&input), Some(3));
}

#[test]
fn test_complex_operation() {
    let input = calculator_input("4 8 + 7 5 - /");
    assert_eq!(evaluate(&input), Some(6));
}

#[test]
fn test_too_few_operands_returns_none() {
    let input = calculator_input("2 +");
    assert_eq!(evaluate(&input), None);
}

#[test]
fn test_too_many_operands_returns_none() {
    let input = calculator_input("2 2");
    assert_eq!(evaluate(&input), None);
}

#[test]
fn test_zero_operands_returns_none() {
    let input = calculator_input("+");
    assert_eq!(evaluate(&input), None);
}

#[test]
fn test_intermediate_error_returns_none() {
    let input = calculator_input("+ 2 2 *");
    assert_eq!(evaluate(&input), None);
}
