use std::collections::HashMap;

pub mod file_reader;
pub mod scanner;


fn main() {

    // debug stuff
    let lex: scanner::Token = scanner::Token {
        token_type: scanner::TokenType::String(String::from("nigga")),
        line: 53
    };

    println!("{}", scanner::Token::to_string(&lex));

    // debug stuff
    println!("{}", file_reader::read_file("nigger.txt"));

    let _unimportant: HashMap<char, scanner::Token> = scanner::_scan_token(3);
}


