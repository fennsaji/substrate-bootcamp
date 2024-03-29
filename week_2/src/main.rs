use std::io;

// Define the Operation enum
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

// Implement the calculate function
fn calculate(operation: &Operation) -> f64 {
    match operation {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => {
            if *b != 0.0 {
                a / b
            } else {
                println!("Error: Division by zero");
                std::f64::NAN
            }
        }
    }
}

fn main() {
    // Prompt the user for input
    println!("Enter the first number:");
    let mut first_number = String::new();
    io::stdin().read_line(&mut first_number).expect("Failed to read line");
    let first_number: f64 = first_number.trim().parse().expect("Invalid input. Please enter a number.");

    println!("Enter the operation (+, -, *, /):");
    let mut operation = String::new();
    io::stdin().read_line(&mut operation).expect("Failed to read line");
    let operation = operation.trim();

    println!("Enter the second number:");
    let mut second_number = String::new();
    io::stdin().read_line(&mut second_number).expect("Failed to read line");
    let second_number: f64 = second_number.trim().parse().expect("Invalid input. Please enter a number.");

    // Create an Operation enum instance
    let operation_enum = match operation {
        "+" => Operation::Add(first_number, second_number),
        "-" => Operation::Subtract(first_number, second_number),
        "*" => Operation::Multiply(first_number, second_number),
        "/" => Operation::Divide(first_number, second_number),
        _ => {
            println!("Invalid operation");
            return;
        }
    };

    // Call the calculate function with the created Operation enum instance
    let result = calculate(&operation_enum);

    // Print the result to the console
    println!("Result: {}", result);
}