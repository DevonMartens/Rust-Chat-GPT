mod ai_functions;
mod apis;
mod models;
mod helpers;

use helpers::command_line::get_user_response;

fn main() {
    let usr_req: String = get_user_response("What is your name?");
    dbg!(usr_req);
}
