/*
In this task, students will create a simple calculator 
in Rust that can perform basic arithmetic operations using enums and pattern matching.
*/

enum Operation  {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64)
} 

fn calculate(op: Operation) -> f64 {
    match op {
        Operation::Add(x1, x2) => {
            x1 + x2
        }
        Operation::Subtract(x1,x2 ) => {
            x1 - x2
        }
        Operation::Multiply(x1,x2 ) => {
            x1 * x2
        }
        Operation::Divide(x1,x2 ) => {
            x1 / x2
        }
    }
}

pub fn calculator() {

    let mut line = String::new();
    println!("Welcome to fastest calculator build on rust! (Please write what you in this format = '3 + 5' or '4.5 * 3.2");
    std::io::stdin().read_line(&mut line).unwrap();

    // Parse operation by inputs, and 
    match parse_operation(&line) {
        Ok(operation) => {
            let result = calculate(operation);
            println!("Result: {}", result);
        }
        Err(error) => {
            println!("Error: {}", error);
        }
    }

}

fn parse_operation(input: &str) -> Result<Operation, String> {
    // Split input into parts
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.len() != 3 {
        return Err("Input must be in the format 'number operator number'.".to_string());
    }

    // Parse the numbers
    let left: f64 = parts[0]
        .parse()
        .map_err(|_| "Failed to parse the first number.".to_string())?;
    let right: f64 = parts[2]
        .parse()
        .map_err(|_| "Failed to parse the second number.".to_string())?;

    // Parse the operator
    match parts[1] {
        "+" => Ok(Operation::Add(left, right)),
        "-" => Ok(Operation::Subtract(left, right)),
        "*" => Ok(Operation::Multiply(left, right)),
        "/" => {
            if right == 0.0 {
                Err("Division by zero is not allowed.".to_string())
            } else {
                Ok(Operation::Divide(left, right))
            }
        }
        _ => Err("Invalid operator. Use one of +, -, *, /.".to_string()),
    }
}