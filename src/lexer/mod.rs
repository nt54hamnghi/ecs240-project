use std::str::Chars;
use std::{iter::Peekable, str::CharIndices};

use crate::lexer::token::{Token, TokenKind};

mod token;

#[derive(Debug)]
pub struct Lexer<'src> {
    source: &'src str,
    inner: Peekable<CharIndices<'src>>,
    line: usize,
}

impl<'src> Lexer<'src> {
    pub fn new(source: &'src str) -> Self {
        let inner = source.char_indices().peekable();
        Self {
            source,
            inner,
            line: 1,
        }
    }

    /// Creates a token from the source text in the byte range `start..end`.
    fn make_token(&self, kind: TokenKind, start: usize, end: usize) -> Token<'src> {
        assert!(start < end);
        Token::new(&self.source[start..end], kind, self.line)
    }

    /// Creates a single-character symbol token at the given byte index.
    fn make_symbol(&self, kind: TokenKind, idx: usize) -> Token<'src> {
        self.make_token(kind, idx, idx + 1)
    }

    fn integer(&mut self) -> Token<'src> {
        todo!()
    }

    fn ident(&mut self) -> Token<'src> {
        todo!()
    }
}

impl<'src> Iterator for Lexer<'src> {
    type Item = Token<'src>;

    fn next(&mut self) -> Option<Self::Item> {
        let (idx, ch) = self.inner.next()?;
        let token = match ch {
            '(' => self.make_symbol(TokenKind::LeftParen, idx),
            ')' => self.make_symbol(TokenKind::RightParen, idx),
            '{' => self.make_symbol(TokenKind::LeftBrace, idx),
            '}' => self.make_symbol(TokenKind::RightBrace, idx),
            ',' => self.make_symbol(TokenKind::Comma, idx),
            ';' => self.make_symbol(TokenKind::Semicolon, idx),
            '&' => self.make_symbol(TokenKind::Ampersand, idx),
            '|' => self.make_symbol(TokenKind::Pipe, idx),
            '+' => self.make_symbol(TokenKind::Plus, idx),
            '-' => self.make_symbol(TokenKind::Minus, idx),
            '*' => self.make_symbol(TokenKind::Star, idx),
            '/' => todo!("division or // line comment"),
            '!' => todo!("! or !="),
            '=' => todo!("= or =="),
            '>' => todo!("> or >="),
            '<' => todo!("< or <="),
            ' ' | '\t' | '\r' => return self.next(),
            '\n' => todo!("advance line number"),
            '0'..='9' => self.integer(),
            'a'..='z' | 'A'..='Z' | '_' => self.ident(),
            _ => todo!("unexpected character"),
        };

        Some(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::left_paren("(", TokenKind::LeftParen)]
    #[case::right_paren(")", TokenKind::RightParen)]
    #[case::left_brace("{", TokenKind::LeftBrace)]
    #[case::right_brace("}", TokenKind::RightBrace)]
    #[case::comma(",", TokenKind::Comma)]
    #[case::semicolon(";", TokenKind::Semicolon)]
    #[case::ampersand("&", TokenKind::Ampersand)]
    #[case::pipe("|", TokenKind::Pipe)]
    #[case::plus("+", TokenKind::Plus)]
    #[case::minus("-", TokenKind::Minus)]
    #[case::star("*", TokenKind::Star)]
    fn yields_symbol_tokens(#[case] symbol: &str, #[case] kind: TokenKind) {
        let mut lexer = Lexer::new(symbol);

        assert_eq!(lexer.next(), Some(Token::new(symbol, kind, 1)));
        assert_eq!(lexer.next(), None);
    }

    #[rstest]
    #[case::spaces(" ")]
    #[case::tabs("\t")]
    #[case::carriage_returns("\r")]
    #[case::mixed(" \t\r")]
    fn skips_whitespace(#[case] whitespace: &str) {
        assert_eq!(Lexer::new(whitespace).next(), None);
    }

    #[rstest]
    #[case::spaces(" ")]
    #[case::tabs("\t")]
    #[case::carriage_returns("\r")]
    #[case::mixed(" \t\r")]
    fn skips_whitespace_around_symbols(#[case] whitespace: &str) {
        let source = format!("{whitespace}+{whitespace}-{whitespace}");
        let tokens: Vec<_> = Lexer::new(&source).collect();
        assert_eq!(
            tokens,
            vec![
                Token::new("+", TokenKind::Plus, 1),
                Token::new("-", TokenKind::Minus, 1),
            ]
        );
    }

    #[rstest]
    #[case("(){},;&|+-*")]
    fn yields_symbols_in_source_order(#[case] source: &str) {
        let tokens: Vec<_> = Lexer::new(source).collect();

        assert_eq!(
            tokens,
            vec![
                Token::new("(", TokenKind::LeftParen, 1),
                Token::new(")", TokenKind::RightParen, 1),
                Token::new("{", TokenKind::LeftBrace, 1),
                Token::new("}", TokenKind::RightBrace, 1),
                Token::new(",", TokenKind::Comma, 1),
                Token::new(";", TokenKind::Semicolon, 1),
                Token::new("&", TokenKind::Ampersand, 1),
                Token::new("|", TokenKind::Pipe, 1),
                Token::new("+", TokenKind::Plus, 1),
                Token::new("-", TokenKind::Minus, 1),
                Token::new("*", TokenKind::Star, 1),
            ]
        );
    }
}
