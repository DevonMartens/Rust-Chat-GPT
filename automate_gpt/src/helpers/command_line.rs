use crossterm::{
    execute,
    ExecutableCommand,
    style::{Color, Print, ResetColor, SetForegroundColor},
}; // used to change the color in commnad line for better user experience
use std::io::{stdin, stdout}; // used to get user input

// get user request
pub fn get_user_response(question: &str) -> String {
    let mut stdout = std::io::stdout();
    // print question
    stdout
        .execute(SetForegroundColor(Color::Cyan))
        .unwrap();
    println!("");
    println!("{}", question);
}