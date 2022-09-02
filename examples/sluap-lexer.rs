use sluap::{Latin1Decoder, Lexer, TokenKind};

fn main() {
    let mut args = std::env::args();
    let _self = args.next().unwrap();
    if let Some(path) = args.next() {
        let bytes = std::fs::read(&path).unwrap();
        let mut lexer = Lexer::new(Latin1Decoder::new(&bytes));
        loop {
            let token = lexer.next_token();
            println!("{:?}", token);

            if token == Ok(TokenKind::Eof) {
                break;
            }
        }
    } else {
        eprintln!("USAGE: {} FILE", _self);
    }
}
