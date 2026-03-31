use std::{collections::HashMap, u128};


// HashMap to convert characters into TokenType's


// Defines all the tokens/lexes
// Used for the scanning process
#[derive(Debug)] // Honestly at this point idk why this has to be here it just doesn't work without it
pub enum TokenType {
    LeftParen, // Single-Character Tokens
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,
    Bang, // One or Two Character Tokens
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Identifier, // Literals
    String(String), // The scanner picks a string from between "" and puts it in the tuple
    Number(u128), // Like the String token but for numbers
    And, // Keywords
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof, // End Of File Token  
}


#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub line: u128,
}

impl Token {
    pub fn to_string(&self) -> String{
        format!("type: {:?}, line: {}", self.token_type, self.line)
    }
}

pub fn _scan_token(current_line: u128) -> HashMap<char, Token> { 
    // HashMap containing characters that may appear in the code
    let mut token_hash: HashMap<char, Token> = HashMap::new();
    token_hash.insert('(', Token {token_type: TokenType::LeftParen, line: current_line}); 
    token_hash.insert(')', Token {token_type: TokenType::RightParen, line: current_line});
    token_hash.insert('{', Token {token_type: TokenType::LeftParen, line: current_line});
    token_hash.insert('}', Token {token_type: TokenType::LeftParen, line: current_line});
    token_hash.insert(',', Token {token_type: TokenType::LeftParen, line: current_line});
    token_hash.insert('.', Token {token_type: TokenType::LeftParen, line: current_line});
    token_hash.insert('-', Token {token_type: TokenType::LeftParen, line: current_line});
    token_hash.insert('+', Token {token_type: TokenType::LeftParen, line: current_line});
    token_hash.insert(';', Token {token_type: TokenType::LeftParen, line: current_line});
    token_hash.insert('*', Token {token_type: TokenType::LeftParen, line: current_line});

    let paren: char = ')';
    println!("{:?}", token_hash.get(&paren));

    token_hash
}