

pub fn extend_ai_function(ai_func: fn(&str) -> &'static str) -> fn(&str) -> &'static str, funct_input &str) -> Message {
    let ai_func_str = ai_func(funct_input);
    // Message {
    //     message: message,
    //     message_type: MessageType::Text,
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extend_ai_function() {
        let ai_func = |input: &str| -> &'static str {
            "Hello World"
        };
        let ai_func_extended = extend_ai_function(ai_func, "Hello World");
        assert_eq!(ai_func_extended, "Hello World");
    }
}