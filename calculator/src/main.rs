use std::io; // Importing standard I/O library for user input
use std::{thread, time::Duration}; // Importing libraries for multi-threading and timing

// Defining an enum to represent the four basic operations
enum Operators {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

// Implementing a method for the Operators enum to perform the corresponding operation
impl Operators {
    fn operations(self, a: i32, b: i32) -> i32 {
        // Matching on the operation type to perform the correct calculation
        match self {
            Operators::Addition => a + b, // Addition
            Operators::Subtraction => a - b, // Subtraction
            Operators::Multiplication => a * b, // Multiplication
            Operators::Division => {
                // Handling division with a check to prevent division by zero
                if b != 0 {
                    a / b
                } else {
                    panic!("Division by zero error");
                }
            }
        }
    }
}

fn main() {
    // Displaying welcome message and instructions
    println!("Loading program...");
    println!("Welcome to my calculator!");
    println!("Pick an operation: ");
    println!("1. Addition");
    println!("2. Subtraction");
    println!("3. Multiplication");
    println!("4. Division");
    println!("Enter your desired operation: ");

    // Getting the user's choice for the operation
    let mut operation_input = String::new();
    io::stdin()
        .read_line(&mut operation_input)
        .expect("Failed to read line");

    // Parsing the input as an integer
    let operation_mode: i32 = operation_input
        .trim()
        .parse()
        .expect("This is not an integer");
    println!("Thank you for your input!");

    // Prompting the user for the first number
    println!("Enter your first number: ");
    let mut first_input = String::new();
    io::stdin()
        .read_line(&mut first_input)
        .expect("Failed to read line");

    // Prompting the user for the second number
    println!("Enter your second number: ");
    let mut second_input = String::new();
    io::stdin()
        .read_line(&mut second_input)
        .expect("Failed to read line");

    // Converting the inputs from strings to integers
    let first_number: i32 = first_input.trim().parse().expect("This is not an integer");
    let second_number: i32 = second_input.trim().parse().expect("This is not an integer");

    // Matching the user's operation choice to the corresponding enum variant
    let operator = match operation_mode {
        1 => Operators::Addition,
        2 => Operators::Subtraction,
        3 => Operators::Multiplication,
        4 => Operators::Division,
        _ => panic!("Invalid operation choice"), // Handling invalid operation input
    };

    println!("Calculating operation...");
    // Performing the calculation
    let result = operator.operations(first_number, second_number);
    
    // Adding a 5-second delay before showing the result to simulate a "cool-down" period
    let cool_down_timer = Duration::from_secs(5);
    thread::sleep(cool_down_timer); // Sleeping the thread for 5 seconds
    
    // Displaying the result of the operation
    println!("Result: {}", result);
}