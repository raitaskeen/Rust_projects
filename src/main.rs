

pub mod backend;

use std::io;
extern crate rand;

fn main() {
    println!("Welcome to the Logic Learning Program!");

    let mut score = 0;

    for _ in 1..=5 {
        let (question, expected_answer) = generate_question();
        println!("Question: {}", question);

        let user_answer: i32 = get_user_input();

        println!("Your answer: {}", user_answer); // Display the user's answer

        if user_answer == expected_answer {
            println!("Correct!");
            score += 1;
        } else {
            println!("Incorrect.");
            println!("The correct answer is: {}", expected_answer);
        }

        println!("---------------------");
    }

    println!("Quiz complete! Your final score is: {}/5", score);
}

fn generate_question() -> (String, i32) {
    let num1 = rand::random::<i32>() % 10 + 1;
    let num2 = rand::random::<i32>() % 10 + 1;

    let operator = match rand::random::<bool>() {
        true => "+",
        false => "-",
    };

    let question = format!("{} {} {}", num1, operator, num2);

    let expected_answer = match operator {
        "+" => num1 + num2,
        "-" => num1 - num2,
        _ => unreachable!(),
    };
 

    (question, expected_answer)
}

fn get_user_input() -> i32 {
    loop {
        let mut user_input = String::new();

        // println!("Your answer: ");

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        match user_input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input. Please enter a number."),
        }
    }
}