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
    pub current: u32,
    pub start: u32,
}

impl ScanToken {
    pub fn scan_token(&self) -> Vec<token::Token> {

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

        // Debug
        let paren: char = '+';
        println!("{:?}", token::Token::to_string(token_hash.get(&paren).unwrap()));

        let mut tokens_list: Vec<token::Token> = Vec::new();
        
        // tokens_list.push(token_hash.get(&paren));        

        match token_hash.get(&paren) {
            None => tokens_list.push(token::Token {token_type: token::TokenType::Minus, line: self.current_line}),
            Some(_x) => tokens_list.push(token::Token {token_type: token::TokenType::Plus, line: self.current_line})
        }



        // Returns the list of tokens which have been scanned
        tokens_list
    }
}