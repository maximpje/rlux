use std::{collections::{HashMap, btree_map::Keys}, u128, vec};




// Defines all the tokens/lexes
// Used for the scanning process
#[derive(Debug)] 
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

pub fn _scan_token(current_line: u128) -> Vec<Token> {

    let tokens_list: Vec<Token> = vec![];

    // Contains the elements that will be put into the HashMap
    let mut char_vec: Vec<char> = vec!['(', ')', '{', '}', ',', '.', '-', '+', ';', '*'];
    let mut token_vec: Vec<Token> = vec![Token {token_type: TokenType::LeftParen, line: current_line}, Token {token_type: TokenType::RightParen, line: current_line}, Token {token_type: TokenType::LeftBrace, line: current_line}, Token {token_type: TokenType::RightBrace, line: current_line}, Token {token_type: TokenType::Comma, line: current_line}, Token {token_type: TokenType::Dot, line: current_line}, Token {token_type: TokenType::Minus, line: current_line}, Token {token_type: TokenType::Plus, line: current_line}, Token {token_type: TokenType::SemiColon, line: current_line}, Token {token_type: TokenType::Star, line: current_line}];
    
    // HashMap containing characters that may appear in the code
    let mut token_hash: HashMap<char, Token> = HashMap::new();

    let mut i = 0;
    while i < 10 {
        token_hash.insert(char_vec.pop().unwrap(), token_vec.pop().unwrap());
        i += 1;
    }

    // Debug
    let paren: char = '+';
    println!("{:?}", token_hash.get(&paren));


    // Returns the list of tokens which have been scanned
    tokens_list
}