use std::io;

struct Question {
    text: String,
    options: Vec<String>,
    correct_answer_index: usize,
}

fn main() {
    println!("Welcome to the Rust Info Quiz!");

    let mut score = 0;

    let questions: Vec<Question> = vec![
        Question {
            text: "Which year was Rust first released?" .to_string(),
            options: vec!["2009".to_string(), "2010".to_string(), "2015".to_string(), "2018".to_string()],
            correct_answer_index: 2,
        },
        Question {
            text: "What ownership system does Rust use?" .to_string(),
            options: vec!["Garbage Collection".to_string(), "Reference Counting".to_string(), "Borrowing".to_string(), "None of the above".to_string()],
            correct_answer_index: 2,
        },
        Question {
            text: "Which of the following is a valid Rust keyword for defining an immutable variable?" .to_string(),
            options: vec!["mut".to_string(), "const".to_string(), "let".to_string(), "var".to_string()],
            correct_answer_index: 2,
        },
        Question {
            text: "What is Rust's primary focus?" .to_string(),
            options: vec!["Concurrency".to_string(), "Memory Safety".to_string(), "Object-Oriented Programming".to_string(), "Dynamic Typing".to_string()],
            correct_answer_index: 1,
        },
        Question {
            text: "Which tool is used for managing Rust projects and dependencies?" .to_string(),
            options: vec!["Cargo".to_string(), "RustBuild".to_string(), "RustPackage".to_string(), "RustHub".to_string()],
            correct_answer_index: 0,
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
