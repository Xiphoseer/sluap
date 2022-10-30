//! # The Parser
//!
//! ## Lifetime conventions
//!
//! In this module:
//! - `'l` refers to the lifetime of a lexer
//! - `'i` refers to the lifetime of the input
use crate::{Decoder, Keyword, Latin1Decoder, Lexer, Symbol, Token, TokenKind, Utf8Decoder};

pub struct Error {}

impl From<super::lexer::Error> for Error {
    fn from(_: super::lexer::Error) -> Self {
        Self {}
    }
}

pub trait Chunk<S> {
    fn push_stmt(&mut self, stmt: S);
}

pub trait If<E, C> {
    fn else_if_clause(&mut self, exp: E, block: C);
    fn else_clause(&mut self, block: C);
}

pub trait Sink {
    type If: If<Self::Expr, Self::Chunk> + Into<Self::Stmt>;
    type Var;
    type VarList;
    type Expr;
    type ExprList;
    type Stmt;
    type Chunk: Chunk<Self::Stmt>;

    fn chunk(&mut self) -> Self::Chunk;
    fn block(&mut self, chunk: Self::Chunk) -> Self::Stmt;
    fn while_stmt(&mut self, expr: Self::Expr, block: Self::Chunk) -> Self::Stmt;
    fn repeat_stmt(&mut self, block: Self::Chunk, expr: Self::Expr) -> Self::Stmt;
    fn if_stmt(&mut self, exp: Self::Expr, block: Self::Chunk) -> Self::If;
}

pub struct Parser<'i, D: Decoder<'i>> {
    inner: Lexer<D>,
    next: Token<&'i D::Slice>,
}

fn next_semantic<'i, D: Decoder<'i>>(inner: &mut Lexer<D>) -> Result<Token<&'i D::Slice>, Error> {
    let mut next = inner.token()?;
    while matches!(next.kind(), TokenKind::Whitespace | TokenKind::Comment) {
        next = inner.token()?;
    }
    Ok(next)
}

impl<'i, D: Decoder<'i>> Parser<'i, D> {
    pub fn new(decoder: D) -> Result<Self, Error> {
        let mut inner = Lexer::new(decoder);
        let next = next_semantic(&mut inner)?;
        Ok(Self { inner, next })
    }

    fn peek(&self) -> Token<&'i D::Slice> {
        self.next
    }

    fn next(&mut self) -> Result<Token<&'i D::Slice>, Error> {
        let old = self.peek();
        self.next = next_semantic(&mut self.inner)?;
        Ok(old)
    }

    pub fn parse_expr<P: Sink>(&mut self, sink: &mut P) -> Result<P::Expr, Error> {
        todo!()
    }

    fn expect_keyword(&mut self, keyword: Keyword) -> Result<(), Error> {
        if TokenKind::Keyword(keyword) == self.next()?.kind() {
            Ok(())
        } else {
            Err(Error { /* missing {keyword} */})
        }
    }

    /// 2.4.2 – Blocks
    fn parse_block<P: Sink>(&mut self, sink: &mut P) -> Result<P::Chunk, Error> {
        if self.next()?.kind() != TokenKind::Keyword(Keyword::Do) {
            return Err(Error { /* missing do at start of block */});
        }
        let chunk = self.parse_chunk(sink)?;
        self.expect_keyword(Keyword::End)?;
        Ok(chunk)
    }

    pub fn parse_stmt<P: Sink>(&mut self, sink: &mut P) -> Result<P::Stmt, Error> {
        // Reminder: There are no empty statements
        match self.peek().kind() {
            TokenKind::Whitespace | TokenKind::Comment => unreachable!(),

            TokenKind::Keyword(Keyword::Do) => {
                let block = self.parse_block(sink)?;
                Ok(sink.block(block))
            }
            TokenKind::Keyword(Keyword::While) => {
                self.next()?; // pop the keyword
                let expr = self.parse_expr(sink)?;
                let block = self.parse_block(sink)?;
                Ok(sink.while_stmt(expr, block))
            }
            TokenKind::Keyword(Keyword::Repeat) => {
                self.next()?; // pop the keyword
                let block = self.parse_chunk(sink)?;
                self.expect_keyword(Keyword::Until)?;
                let expr = self.parse_expr(sink)?;
                Ok(sink.repeat_stmt(block, expr))
            }
            TokenKind::Keyword(Keyword::If) => {
                self.next()?; // pop the keyword
                let exp = self.parse_expr(sink)?;
                self.expect_keyword(Keyword::Then)?;
                let block = self.parse_chunk(sink)?;
                let mut builder = sink.if_stmt(exp, block);

                let mut next = self.next()?;
                loop {
                    if next.kind() == TokenKind::Keyword(Keyword::ElseIf) {
                        let exp = self.parse_expr(sink)?;
                        self.expect_keyword(Keyword::Then)?;
                        let block = self.parse_chunk(sink)?;
                        builder.else_if_clause(exp, block);
                        next = self.next()?;
                    } else {
                        break;
                    }
                }
                if next.kind() == TokenKind::Keyword(Keyword::Else) {
                    let block = self.parse_chunk(sink)?;
                    builder.else_clause(block);
                }
                Ok(builder.into())
            }
            TokenKind::Keyword(_) => todo!(),
            TokenKind::Name => todo!(),
            TokenKind::Symbol(_) => todo!(),
            TokenKind::String => todo!(),
            TokenKind::Hex(_) => todo!(),
            TokenKind::Number(_) => todo!(),
            TokenKind::Eof => todo!(),
        }
    }

    pub fn parse_chunk<P: Sink>(&mut self, sink: &mut P) -> Result<P::Chunk, Error> {
        let mut chunk: P::Chunk = sink.chunk();
        loop {
            match self.peek().kind() {
                TokenKind::Keyword(
                    Keyword::End | Keyword::Until | Keyword::Else | Keyword::ElseIf,
                )
                | TokenKind::Eof => break Ok(chunk),
                TokenKind::Whitespace | TokenKind::Comment => unreachable!(),
                _ => {
                    let stmt = self.parse_stmt(sink)?;
                    if self.peek().kind() == TokenKind::Symbol(Symbol::Semicolon) {
                        self.next()?;
                    }
                    chunk.push_stmt(stmt);
                }
            }
        }
    }
}

impl<'i> Parser<'i, Utf8Decoder<'i>> {
    pub fn new_from_str(text: &'i str) -> Result<Self, Error> {
        Self::new(Utf8Decoder::new(text))
    }
}

impl<'i> Parser<'i, Latin1Decoder<'i>> {
    pub fn new_from_latin1(bytes: &'i [u8]) -> Result<Self, Error> {
        Self::new(Latin1Decoder::new(bytes))
    }
}
