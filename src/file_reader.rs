use std::fs;

// Reads from a file
pub fn read_file(input_path: &str)  -> String { 
    match fs::read_to_string(String::from(input_path)){
        Ok(answer) => return answer,
        Err(e) => return format!("Error: {}", e),

    };
}