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

    fn make_token_range(&self, kind: TokenKind, start: usize, end: usize) -> Token<'src> {
        assert!(start < end);
        Token::new(&self.source[start..end], kind, self.line)
    }

    /// Creates a single-character symbol token at the given byte index.
    fn make_token_len(&self, kind: TokenKind, start: usize, len: usize) -> Token<'src> {
        self.make_token_range(kind, start, start + len)
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
            '(' => self.make_token_len(TokenKind::LeftParen, idx, 1),
            ')' => self.make_token_len(TokenKind::RightParen, idx, 1),
            '{' => self.make_token_len(TokenKind::LeftBrace, idx, 1),
            '}' => self.make_token_len(TokenKind::RightBrace, idx, 1),
            ',' => self.make_token_len(TokenKind::Comma, idx, 1),
            ';' => self.make_token_len(TokenKind::Semicolon, idx, 1),
            '&' => self.make_token_len(TokenKind::Ampersand, idx, 1),
            '|' => self.make_token_len(TokenKind::Pipe, idx, 1),
            '+' => self.make_token_len(TokenKind::Plus, idx, 1),
            '-' => self.make_token_len(TokenKind::Minus, idx, 1),
            '*' => self.make_token_len(TokenKind::Star, idx, 1),
            '/' => todo!("division or // line comment"),
            '!' => match self.inner.next_if(|(_, c)| *c == '=') {
                Some(_) => self.make_token_len(TokenKind::BangEqual, idx, 2),
                None => self.make_token_len(TokenKind::Bang, idx, 1),
            },
            '=' => match self.inner.next_if(|(_, c)| *c == '=') {
                Some(_) => self.make_token_len(TokenKind::EqualEqual, idx, 2),
                None => self.make_token_len(TokenKind::Equal, idx, 1),
            },
            '>' => match self.inner.next_if(|(_, c)| *c == '=') {
                Some(_) => self.make_token_len(TokenKind::GreaterEqual, idx, 2),
                None => self.make_token_len(TokenKind::Greater, idx, 1),
            },
            '<' => match self.inner.next_if(|(_, c)| *c == '=') {
                Some(_) => self.make_token_len(TokenKind::LessEqual, idx, 2),
                None => self.make_token_len(TokenKind::Less, idx, 1),
            },
            ' ' | '\t' | '\r' => return self.next(),
            '\n' => {
                self.line += 1;
                return self.next();
            }
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
    #[case::bang("!", TokenKind::Bang)]
    #[case::bang_equal("!=", TokenKind::BangEqual)]
    #[case::equal("=", TokenKind::Equal)]
    #[case::equal_equal("==", TokenKind::EqualEqual)]
    #[case::greater(">", TokenKind::Greater)]
    #[case::greater_equal(">=", TokenKind::GreaterEqual)]
    #[case::less("<", TokenKind::Less)]
    #[case::less_equal("<=", TokenKind::LessEqual)]
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

    #[test]
    fn advances_line_number_on_newline() {
        let tokens: Vec<_> = Lexer::new("+\n\n-").collect();
        assert_eq!(
            tokens,
            vec![
                Token::new("+", TokenKind::Plus, 1),
                Token::new("-", TokenKind::Minus, 3),
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
