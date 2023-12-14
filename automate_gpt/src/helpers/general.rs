use crate::models::general::llm::Message;
use crate::helpers::command_line::PrintCommand;

pub fn extend_ai_function(ai_func: fn(&str) -> &'static str) -> fn(&str) -> &'static str, funct_input &str) -> Message {
    let ai_func_str = ai_func(funct_input);
    let msg: String = format!("function {}  Instruction: funct printer. With input {}", ai_func_str, funct_input);
    // return msg;
    Message {
        role: "system".to_string(),
        content: msg,
    }
}

pub fn ai_task_request(
    msg_context: String,
    agent_pos: &str,
    agent_operation: &str,
    function_pass: fn(&str) -> &'static str,
) -> Srting {
    let func_msg: Message = extend_ai_function(ai_func: function_pass, funct_input&msg_context);

    PrintCommand::AICall.print_agent_message(agent_pos, agent_operation);

    String::from("Hello World")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;

    #[test]
    fn test_extend_ai_function() {
        let ai_func = |input: &str| -> &'static str {
            "Hello World"
        };
        let ai_func_extended = extend_ai_function(ai_func, "Hello World");
        assert_eq!(ai_func_extended, "Hello World");
    }

    let x_str = convert_user_input_to_goal(
        //_user_request: 
        "Hello World");
}