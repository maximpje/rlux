use std::{collections::{HashMap}, u128, vec};
// use std::sync::atomic;

pub mod scanner_helpers;


// scanner variables
//static mut LINE: atomic::AtomicU32 = atomic::AtomicU32::new(1);  // current line where it's scanning
//static mut CURRENT: atomic::AtomicU32 = atomic::AtomicU32::new(0); // current character scanning 
//static mut START: atomic::AtomicU32 = atomic::AtomicU32::new(0);



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

// Token object definiton
#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub line: u32,
}

impl Token {
    pub fn to_string(&self) -> String{
        format!("type: {:?}, line: {}", self.token_type, self.line)
    }
}


// Scanner object
#[derive(Debug)]
pub struct ScanToken {
    pub current_line: u32,
    pub current: u32,
    pub start: u32,
}

impl ScanToken {
    pub fn scan_token(&self) -> Vec<Token> {

        // Contains the elements that will be put into the HashMap
        let mut char_vec: Vec<char> = vec!['(', ')', '{', '}', ',', '.', '-', '+', ';', '*'];
        let mut token_vec: Vec<Token> = vec![Token {token_type: TokenType::LeftParen, line: self.current_line}, Token {token_type: TokenType::RightParen, line: self.current_line}, Token {token_type: TokenType::LeftBrace, line: self.current_line}, Token {token_type: TokenType::RightBrace, line: self.current_line}, Token {token_type: TokenType::Comma, line: self.current_line}, Token {token_type: TokenType::Dot, line: self.current_line}, Token {token_type: TokenType::Minus, line: self.current_line}, Token {token_type: TokenType::Plus, line: self.current_line}, Token {token_type: TokenType::SemiColon, line: self.current_line}, Token {token_type: TokenType::Star, line: self.current_line}];
        
        // HashMap containing characters that may appear in the code
        let mut token_hash: HashMap<char, Token> = HashMap::new();

        let mut i = 0;
        while i < 10 {
            token_hash.insert(char_vec.pop().unwrap(), token_vec.pop().unwrap());
            i += 1;
        }

        // Debug
        let paren: char = '+';
        println!("{:?}", Token::to_string(token_hash.get(&paren).unwrap()));

        let mut tokens_list: Vec<Token> = Vec::new();
        
        // tokens_list.push(token_hash.get(&paren));        

        match token_hash.get(&paren) {
            None => tokens_list.push(Token {token_type: TokenType::Minus, line: self.current_line}),
            Some(_x) => tokens_list.push(Token {token_type: TokenType::Plus, line: self.current_line})
        }



        // Returns the list of tokens which have been scanned
        tokens_list
    }
}