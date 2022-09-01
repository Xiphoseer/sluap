use lexical_core::{format::STANDARD, NumberFormatBuilder, ParseFloatOptions, ParseIntegerOptions};
use memchr::{memchr, memchr2};
use unicode_xid::UnicodeXID;

use crate::{tokens::Symbol, Decoder, Keyword, TokenKind};

pub struct Lexer<D> {
    decoder: D,
    float_options: ParseFloatOptions,
    int_options: ParseIntegerOptions,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Error {}

fn is_string_block<'a, D: Decoder<'a>>(decoder: &D) -> Option<(usize, D)> {
    let mut test = decoder.clone();
    if let Some('[') = test.next_char() {
        let mut level = 0;
        while let Some('=') = test.next_char() {
            level += 1;
        }
        if let Some('[') = test.next_char() {
            return Some((level, test));
        }
    }
    None
}

fn scan_to_string_end<'a, D: Decoder<'a>>(test: &mut D, level: usize) -> Result<(), Error> {
    loop {
        if let Some(skip) = memchr(b']', test.as_bytes()) {
            test.skip_bytes(skip);
            let mut next = test.next_char();

            // This should always succeed the first time around
            while let Some(']') = next {
                next = test.next_char();
                let mut end_level = 0;
                while let Some('=') = next {
                    end_level += 1;
                    next = test.next_char();
                }
                if end_level == level && next == Some(']') {
                    return Ok(());
                }
            }
        } else {
            // Missing end of block comment
            return Err(Error {});
        }
    }
}

impl<'a, D: Decoder<'a>> Lexer<D> {
    pub fn new(decoder: D) -> Self {
        let float_options = ParseFloatOptions::builder()
            .lossy(true)
            .nan_string(None)
            .inf_string(None)
            .infinity_string(None)
            .build()
            .unwrap();
        let int_options = ParseIntegerOptions::new();
        Self {
            decoder,
            float_options,
            int_options,
        }
    }

    fn next_char(&mut self) -> Option<char> {
        self.decoder.next_char()
    }

    fn pop_peeked(&mut self) {
        self.decoder.next_char();
    }

    fn peek_char(&mut self) -> Option<char> {
        self.decoder.peek_char()
    }

    pub fn next_token(&mut self) -> Result<TokenKind, Error> {
        let start = self.decoder.as_bytes();
        match self.next_char() {
            Some('+') => Ok(TokenKind::Symbol(Symbol::Plus)),
            Some('-') => match self.peek_char() {
                Some('-') => {
                    self.pop_peeked();
                    // At this point, we're sure we're in a comment
                    let _c_start = self.decoder.as_bytes();

                    if let Some((level, mut test)) = is_string_block(&self.decoder) {
                        let _block_start = test.as_bytes();
                        scan_to_string_end(&mut test, level)?;
                        self.decoder = test;
                    } else {
                        // This is a standard newline comment
                        let newline = memchr2(b'\n', b'\r', _c_start).unwrap_or(_c_start.len());
                        // The comment token ends just before the first newline character
                        self.decoder.skip_bytes(newline);
                    }
                    Ok(TokenKind::Comment)
                }
                _ => Ok(TokenKind::Symbol(Symbol::Minus)),
            },
            Some('*') => Ok(TokenKind::Symbol(Symbol::Times)),
            Some('/') => Ok(TokenKind::Symbol(Symbol::Slash)),
            Some('%') => Ok(TokenKind::Symbol(Symbol::Percent)),
            Some('^') => Ok(TokenKind::Symbol(Symbol::Caret)),
            Some('#') => Ok(TokenKind::Symbol(Symbol::Hash)),
            Some('=') => match self.peek_char() {
                Some('=') => {
                    self.pop_peeked();
                    Ok(TokenKind::Symbol(Symbol::Eq))
                }
                _ => Ok(TokenKind::Symbol(Symbol::Assign)),
            },
            Some('~') => match self.next_char() {
                Some('=') => Ok(TokenKind::Symbol(Symbol::NotEq)),
                _ => Err(Error {}),
            },
            Some('<') => match self.peek_char() {
                Some('=') => {
                    self.pop_peeked();
                    Ok(TokenKind::Symbol(Symbol::LtEq))
                }
                _ => Ok(TokenKind::Symbol(Symbol::Lt)),
            },
            Some('>') => match self.peek_char() {
                Some('=') => {
                    self.pop_peeked();
                    Ok(TokenKind::Symbol(Symbol::GtEq))
                }
                _ => Ok(TokenKind::Symbol(Symbol::Gt)),
            },
            Some('(') => Ok(TokenKind::Symbol(Symbol::ParenL)),
            Some(')') => Ok(TokenKind::Symbol(Symbol::ParenR)),
            Some('{') => Ok(TokenKind::Symbol(Symbol::BraceL)),
            Some('}') => Ok(TokenKind::Symbol(Symbol::BraceR)),
            Some('[') => match self.peek_char() {
                Some('[') => todo!(),
                Some('=') => todo!(),
                _ => Ok(TokenKind::Symbol(Symbol::BracketL)),
            },
            Some(']') => Ok(TokenKind::Symbol(Symbol::BracketR)),
            Some(';') => Ok(TokenKind::Symbol(Symbol::Semicolon)),
            Some(':') => Ok(TokenKind::Symbol(Symbol::Colon)),
            Some(',') => Ok(TokenKind::Symbol(Symbol::Comma)),
            Some('.') => match self.peek_char() {
                Some('.') => {
                    self.pop_peeked();
                    match self.peek_char() {
                        Some('.') => {
                            self.pop_peeked();
                            Ok(TokenKind::Symbol(Symbol::Dot3))
                        }
                        _ => Ok(TokenKind::Symbol(Symbol::Dot2)),
                    }
                }
                _ => Ok(TokenKind::Symbol(Symbol::Dot)),
            },
            Some('\'') => todo!("string '"),
            Some('"') => todo!("string \""),
            Some(c) if c.is_whitespace() => {
                while let Some(c) = self.peek_char() {
                    if c.is_whitespace() {
                        self.pop_peeked();
                    } else {
                        break;
                    }
                }
                // could return EOF here, but we had at least one whitespace
                Ok(TokenKind::Whitespace)
            }
            Some(c) if c.is_xid_start() => {
                while let Some(c) = self.peek_char() {
                    if c.is_xid_continue() {
                        self.pop_peeked();
                    } else {
                        break;
                    }
                }
                let len = self.decoder.offset_from(start);
                let (bytes, _) = start.split_at(len as usize);
                if let Some(keyword) = Keyword::from_bytes(bytes) {
                    Ok(TokenKind::Keyword(keyword))
                } else {
                    Ok(TokenKind::Name)
                }
            }
            Some(c) if c.is_ascii_digit() => match self.peek_char() {
                Some('x') => {
                    self.pop_peeked();
                    let input = self.decoder.as_bytes();

                    const HEX: u128 = NumberFormatBuilder::hexadecimal();
                    let (x, len) = lexical_core::parse_partial_with_options::<u32, HEX>(
                        input,
                        &self.int_options,
                    )
                    .map_err(|_e| Error {})?;
                    let skip = len - self.decoder.offset_from(input);
                    self.decoder.skip_bytes(skip);
                    Ok(TokenKind::Hex(x))
                }
                _ => {
                    let (x, len) = lexical_core::parse_partial_with_options::<f32, STANDARD>(
                        start,
                        &self.float_options,
                    )
                    .map_err(|_e| Error {})?;
                    let skip = len - self.decoder.offset_from(start);
                    self.decoder.skip_bytes(skip);
                    Ok(TokenKind::Number(x))
                }
            },
            Some(_) => Err(Error {}),
            None => Ok(TokenKind::Eof),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{tokens::Symbol, Keyword, Latin1Decoder, Lexer, TokenKind};

    #[test]
    fn test_keywords() {
        let decoder = Latin1Decoder::new(b"and or then true");
        let mut lexer = Lexer::new(decoder);

        assert_eq!(lexer.next_token(), Ok(TokenKind::Keyword(Keyword::And)));
        assert_eq!(lexer.next_token(), Ok(TokenKind::Whitespace));
        assert_eq!(lexer.next_token(), Ok(TokenKind::Keyword(Keyword::Or)));
        assert_eq!(lexer.next_token(), Ok(TokenKind::Whitespace));
        assert_eq!(lexer.next_token(), Ok(TokenKind::Keyword(Keyword::Then)));
        assert_eq!(lexer.next_token(), Ok(TokenKind::Whitespace));
        assert_eq!(lexer.next_token(), Ok(TokenKind::Keyword(Keyword::True)));
        assert_eq!(lexer.next_token(), Ok(TokenKind::Eof));
    }

    #[test]
    fn test_symbols() {
        let decoder = Latin1Decoder::new(b"+-*/%");
        let mut lexer = Lexer::new(decoder);

        assert_eq!(lexer.next_token(), Ok(TokenKind::Symbol(Symbol::Plus)));
        assert_eq!(lexer.next_token(), Ok(TokenKind::Symbol(Symbol::Minus)));
        assert_eq!(lexer.next_token(), Ok(TokenKind::Symbol(Symbol::Times)));
        assert_eq!(lexer.next_token(), Ok(TokenKind::Symbol(Symbol::Slash)));
        assert_eq!(lexer.next_token(), Ok(TokenKind::Symbol(Symbol::Percent)));
        assert_eq!(lexer.next_token(), Ok(TokenKind::Eof));
    }

    #[test]
    #[allow(clippy::approx_constant)]
    fn test_numbers() {
        let decoder = Latin1Decoder::new(b"3   3.0   3.1416   314.16e-2   0.31416E1   0xff   0x56");
        let mut lexer = Lexer::new(decoder);

        assert_eq!(lexer.next_token(), Ok(TokenKind::Number(3.0)));
        assert_eq!(lexer.next_token(), Ok(TokenKind::Whitespace));
        assert_eq!(lexer.next_token(), Ok(TokenKind::Number(3.0)));
        assert_eq!(lexer.next_token(), Ok(TokenKind::Whitespace));
        assert_eq!(lexer.next_token(), Ok(TokenKind::Number(3.1416)));
        assert_eq!(lexer.next_token(), Ok(TokenKind::Whitespace));
        assert_eq!(lexer.next_token(), Ok(TokenKind::Number(3.1416)));
        assert_eq!(lexer.next_token(), Ok(TokenKind::Whitespace));
        assert_eq!(lexer.next_token(), Ok(TokenKind::Number(3.1416)));
        assert_eq!(lexer.next_token(), Ok(TokenKind::Whitespace));
        assert_eq!(lexer.next_token(), Ok(TokenKind::Hex(0xFF)));
        assert_eq!(lexer.next_token(), Ok(TokenKind::Whitespace));
        assert_eq!(lexer.next_token(), Ok(TokenKind::Hex(0x56)));
        assert_eq!(lexer.next_token(), Ok(TokenKind::Eof));
    }
}
