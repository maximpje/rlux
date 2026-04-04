pub mod file_reader;
pub mod scanner;


fn main() {

    // debug stuff
    println!("{}", file_reader::read_file("testfile.txt"));

    let _unimportant: scanner::ScanToken = scanner::ScanToken {current_line: 1, current: 0, start: 0};

    _unimportant.scan_token();    
    println!("{:?}", _unimportant);

}


// A message to Subaru if you happend to come here once more:
//  "blud."
