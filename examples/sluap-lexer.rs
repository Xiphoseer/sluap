use sluap::{Latin1Decoded, Latin1Decoder, Lexer, TokenKind};

fn main() {
    let mut args = std::env::args();
    let _self = args.next().unwrap();
    if let Some(path) = args.next() {
        let bytes = std::fs::read(&path).unwrap();
        let mut lexer = Lexer::new(Latin1Decoder::new(&bytes));
        loop {
            let token = lexer.token().unwrap();
            match token.kind() {
                TokenKind::Whitespace => { /* ignore */ }
                TokenKind::Keyword(kw) => println!("Keyword: {:?}", kw),
                TokenKind::Name => println!("Name: {}", Latin1Decoded(token.span())),
                TokenKind::Symbol(sym) => println!("Symbol: {:?}", sym),
                TokenKind::Comment => { /* ignore */ }
                TokenKind::String => println!("String: {}", Latin1Decoded(token.span())),
                TokenKind::Hex(val) => println!("Hex: 0x{:x}", val),
                TokenKind::Number(val) => println!("Number: {}", val),
                TokenKind::Eof => break,
            }
        }
    } else {
        eprintln!("USAGE: {} FILE", _self);
    }
}
