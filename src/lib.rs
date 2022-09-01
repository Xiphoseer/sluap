#![no_std]

mod encoding;
pub use encoding::{Decoder, Latin1Decoder, Utf8Decoder};
mod keywords;
pub use keywords::Keyword;
mod tokens;
pub use tokens::{Symbol, TokenKind};
mod lexer;
pub use lexer::{Error, Lexer};
