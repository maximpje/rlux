use std::{collections::{HashMap}, vec};
// use std::sync::atomic;

pub mod scanner_helpers;
pub mod token;
// scanner variables
//static mut LINE: atomic::AtomicU32 = atomic::AtomicU32::new(1);  // current line where it's scanning
//static mut CURRENT: atomic::AtomicU32 = atomic::AtomicU32::new(0); // current character scanning 
//static mut START: atomic::AtomicU32 = atomic::AtomicU32::new(0);




// Scanner object
#[derive(Debug)]
pub struct ScanToken {
    pub current_line: u32,
    current: u32,
    start: u32,
    source: String,
    tokens: Vec<token::Token>
}

impl ScanToken {
pub fn scan_token(self) -> token::Token {

        // Contains the elements that will be put into the HashMap
        let mut char_vec: Vec<char> = vec!['(', ')', '{', '}', ',', '.', '-', '+', ';', '*'];
        let mut token_vec: Vec<token::Token> = vec![token::Token {token_type: token::TokenType::LeftParen, line: self.current_line}, token::Token {token_type: token::TokenType::RightParen, line: self.current_line}, token::Token {token_type: token::TokenType::LeftBrace, line: self.current_line}, token::Token {token_type: token::TokenType::RightBrace, line: self.current_line}, token::Token {token_type: token::TokenType::Comma, line: self.current_line}, token::Token {token_type: token::TokenType::Dot, line: self.current_line}, token::Token {token_type: token::TokenType::Minus, line: self.current_line}, token::Token {token_type: token::TokenType::Plus, line: self.current_line}, token::Token {token_type: token::TokenType::SemiColon, line: self.current_line}, token::Token {token_type: token::TokenType::Star, line: self.current_line}];
        
        // HashMap containing characters that may appear in the code
        let mut token_hash: HashMap<char, token::Token> = HashMap::new();

        let mut i = 0;
        while i < 10 {
            token_hash.insert(char_vec.pop().unwrap(), token_vec.pop().unwrap());
            i += 1;
        }

        // Scans current token and returns        
        let a: char = self.advance();

        match token_hash.get(&a) {
            None => return token::Token { token_type: token::TokenType::Eof, line: 0 },
            Some(_x) => return token::Token {token_type: token::TokenType::Plus, line: self.current_line.clone()}
        }

    }

    // Returns true if all the tokens have been scanned  
    fn is_at_end(self) -> bool {
        self.current >= self.source.len().try_into().unwrap()
    }

    // Returns the current character to be scanned and increases the iterator
    fn advance(mut self) -> char {
        self.current+=1;
        self.source.chars().nth(self.current.try_into().unwrap()).unwrap()
    }

    // Returns the current line
    fn get_line(self) -> u32{
        self.current_line
    }
}