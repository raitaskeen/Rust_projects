use std::io;

struct Question {
    text: String,
    options: Vec<String>,
    correct_answer_index: usize,
}

fn main() {
    println!("Welcome to the General Knowledge Quiz!");

    let mut score = 0;

    let questions: Vec<Question> = vec![
        Question {
            text: "What is the capital of France?".to_string(),
            options: vec!["Berlin".to_string(), "Paris".to_string(), "Madrid".to_string(), "Rome".to_string()],
            correct_answer_index: 1,
        },
        Question {
            text: "Which planet is known as the Red Planet?".to_string(),
            options: vec!["Mars".to_string(), "Venus".to_string(), "Jupiter".to_string(), "Saturn".to_string()],
            correct_answer_index: 0,
        },
        Question {
            text: "In which year did the Titanic sink?".to_string(),
            options: vec!["1905".to_string(), "1912".to_string(), "1920".to_string(), "1931".to_string()],
            correct_answer_index: 1,
        },
        Question {
            text: "Who painted the Mona Lisa?".to_string(),
            options: vec!["Vincent van Gogh".to_string(), "Leonardo da Vinci".to_string(), "Pablo Picasso".to_string(), "Claude Monet".to_string()],
            correct_answer_index: 1,
        },
        Question {
            text: "What is the largest mammal in the world?".to_string(),
            options: vec!["Elephant".to_string(), "Giraffe".to_string(), "Blue Whale".to_string(), "Hippopotamus".to_string()],
            correct_answer_index: 2,
        },
    ];

    for question in &questions {
        println!("Question: {}", question.text);

        for (index, option) in question.options.iter().enumerate() {
            println!("{}. {}", index + 1, option);
        }

        let user_answer_index = get_user_input(question.options.len());

        println!("Your answer: {}", question.options[user_answer_index - 1]); // Display the user's answer

        if user_answer_index - 1 == question.correct_answer_index {
            println!("Correct!");
            score += 1;
        } else {
            println!("Incorrect. The correct answer is: {}", question.options[question.correct_answer_index]);
        }

        println!("---------------------");
    }

    println!("Quiz complete! Your final score is: {}/{}", score, questions.len());
}

fn get_user_input(max_options: usize) -> usize {
    loop {
        let mut user_input = String::new();

        io::stdin().read_line(&mut user_input).expect("Failed to read line");

        match user_input.trim().parse() {
            Ok(num) if num >= 1 && num <= max_options => return num,
            _ => println!("Invalid input. Please enter a number between 1 and {}.", max_options),
        }
    }
}
