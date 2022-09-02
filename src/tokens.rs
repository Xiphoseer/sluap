//! # Syntax Tokens

use crate::Keyword;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Token<'a, S: ?Sized> {
    span: &'a S,
    kind: TokenKind,
}

impl<'a, S: ?Sized> Token<'a, S> {
    pub fn new(span: &'a S, kind: TokenKind) -> Self {
        Self { span, kind }
    }

    pub fn kind(&self) -> TokenKind {
        self.kind
    }

    pub fn span(&self) -> &'a S {
        self.span
    }
}

/// Output of the lexer
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    /// Anything that matches [char::is_whitespace]
    Whitespace,
    /// Reserved *names* in LUA
    Keyword(Keyword),
    /// Identifiers (based on unicode XID)
    Name,
    /// Symbolic tokens
    Symbol(Symbol),
    /// A comment
    Comment,
    /// A string (with escaping)
    String,
    /// A hexadecimal integer literal
    Hex(u32),
    /// A floating point literal
    Number(f32),
    /// End of input
    Eof,
}

/// Symbolic Tokens
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Symbol {
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `*`
    Times,
    /// `/`
    Slash,
    /// `%`
    Percent,
    /// `^`
    Caret,
    /// `#`
    Hash,
    /// `==`
    Eq,
    /// `~=`
    NotEq,
    /// `<=`
    LtEq,
    /// `>=`
    GtEq,
    /// `<`
    Lt,
    /// `>`
    Gt,
    /// `=`
    Assign,
    /// `(`
    ParenL,
    /// `)`
    ParenR,
    /// `{`
    BraceL,
    /// `}`
    BraceR,
    /// `[`
    BracketL,
    /// `]`
    BracketR,
    /// `;`
    Semicolon,
    /// `:`
    Colon,
    /// `,`
    Comma,
    /// `.`
    Dot,
    /// `..`
    Dot2,
    /// `...`
    Dot3,
}
