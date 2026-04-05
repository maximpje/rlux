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

