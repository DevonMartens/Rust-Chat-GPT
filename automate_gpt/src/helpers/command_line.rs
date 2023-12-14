use crossterm::{
    ExecutableCommand,
    style::{Color, ResetColor, SetForegroundColor},
}; // used to change the color in commnad line for better user experience
use std::io::{stdin, stdout}; // used to get user input

pub enum PrintCommand {
    AICall,
    UnitTest,
    Issue
}

impl PrintCommand {
    pub fn print_agent_message(&self, agent_pos: &str, agent_statment: &str) {
        let mut stdout = stdout();

        let statement_color = match self {
            PrintCommand::AICall => Color::Cyan,
            PrintCommand::UnitTest => Color::Yellow,
            PrintCommand::Issue => Color::Red,
        };
        // print agent position
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        print!("{}: ", agent_pos);

        // print agent statement
        stdout.execute(SetForegroundColor(statement_color)).unwrap();
        println!("{}", agent_statment);

        // reset color
        stdout
            .execute(ResetColor)
            .unwrap();
    }

// get user request
pub fn get_user_response(question: &str) -> String {
    let mut stdout = stdout();
    // print question
    stdout.execute(SetForegroundColor(Color::Cyan)).unwrap();
    println!("");
    println!("{}", question);

    // reset color
    stdout
        .execute(ResetColor)
        .unwrap();

    // get user input
    let mut user_response = String::new();
    stdin()
        .read_line(&mut user_response)
        .expect("Failed to read line");

    // return user input`
    user_response.trim().to_string()

}