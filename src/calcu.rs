fn main() {
    // Loop for 5 steps
    for _ in 0..5 {
        let num1: i32 = 10;
        let num2: i32 = 5;

        // Addition
        let sum = num1 + num2;
        println!("{} + {} = {}", num1, num2, sum);

        // Subtraction
        let diff = num1 - num2;
        println!("{} - {} = {}", num1, num2, diff);

        // Multiplication
        let product = num1 * num2;
        println!("{} * {} = {}", num1, num2, product);

        // Division
        let quotient = num1 / num2;
        println!("{} / {} = {}", num1, num2, quotient);

        println!("------------------");
    }
}
