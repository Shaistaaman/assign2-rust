use std::io::{self, Read};

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(operation: Operation) -> f64 {
    match operation {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => {
            if b == 0.0 {
                panic!("Division by zero is not allowed!");
            }
            a / b
        }
    }
}

fn main() {
    let mut input = String::new();

    println!("Enter the first number:");
    io::stdin().read_line(&mut input).unwrap();
    let first_number = input.trim().parse::<f64>().unwrap();
    input.clear();

    println!("Enter the operation (+, -, *, /):");
    io::stdin().read_line(&mut input).unwrap();
    let operation = match input.trim() {
        "+" => Operation::Add(first_number, 0.0),
        "-" => Operation::Subtract(first_number, 0.0),
        "*" => Operation::Multiply(first_number, 0.0),
        "/" => Operation::Divide(first_number, 0.0),
        _ => panic!("Invalid operation!"),
    };
    input.clear();

    println!("Enter the second number:");
    io::stdin().read_line(&mut input).unwrap();
    let second_number = input.trim().parse::<f64>().unwrap();

    let result = calculate(match operation {
        Operation::Add(_, b) => Operation::Add(first_number, b),
        Operation::Subtract(_, b) => Operation::Subtract(first_number, b),
        Operation::Multiply(_, b) => Operation::Multiply(first_number, b),
        Operation::Divide(_, b) => Operation::Divide(first_number, b),
    });

    println!("The result is: {}", result);
}