use std::io;

fn main() {
    // Prompt the user for a number
    println!("Enter a number to generate its multiplication table:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse the input into an integer
    let number: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return;
        }
    };

    // Generate and print the multiplication table
    println!("Multiplication table for {}:", number);
    for i in 1..=10 {
        println!("{} x {} = {}", number, i, number * i);
    }
}
