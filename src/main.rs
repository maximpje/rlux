pub mod file_reader;
pub mod scanner;


fn main() {

    // debug stuff
    let lex: scanner::Token = scanner::Token {
        token_type: scanner::TokenType::String(String::from("Hello, world!")),
        line: 53
    };

    println!("{}", scanner::Token::to_string(&lex));

    // debug stuff
    println!("{}", file_reader::read_file("testfile.txt"));

    let _unimportant: Vec<scanner::Token> = scanner::_scan_token(3);
}


// A message to Subaru if you happend to come here once more:
//  "blud."
