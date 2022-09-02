#![no_std]
//! # sluap - The Slice LUA Parser
//!
//! This is a `no_std` crate that implements parsing of Lua 5.1
//!
//! See: <https://www.lua.org/manual/5.1/manual.html>

mod encoding;
pub use encoding::{Decoder, Latin1Decoder, Utf8Decoder};
mod keywords;
pub use keywords::Keyword;
mod tokens;
pub use tokens::{Symbol, Token, TokenKind};
mod lexer;
pub use lexer::{Error, Lexer};
