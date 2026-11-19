use std::str::Chars;
use std::{iter::Peekable, str::CharIndices};

use crate::lexer::token::{Token, TokenKind};

mod token;

#[derive(Debug)]
pub struct Lexer<'src> {
    // Stores a reference to the original source code
    source: &'src str,
    // Tracks which character to read next
    // CharIndices returns a Some(&(byte_index, character))
    inner: Peekable<CharIndices<'src>>,
    // Tracks the current source line, starting at 1
    line: usize,
    at_eof: bool,
}

impl<'src> Lexer<'src> {
    pub fn new(source: &'src str) -> Self {
        // char_indices creates an itereator,
        // peakable wraps the iterator so you can look at the next char without consuming it.
        let inner = source.char_indices().peekable();
        Self {
            source,
            inner,
            line: 1,
            at_eof: false,
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

    fn integer(&mut self, start: usize) -> Token<'src> {
        // Get consecutive digit(s) to create a token
        let mut len = 1;
        while let Some((_, ch)) = self.inner.peek()
            && ch.is_ascii_digit()
        {
            len += 1;
            self.inner.next();
        }

        // Create a token of Integer
        self.make_token_len(TokenKind::Integer, start, len)
    }

    fn ident_or_keyword(&mut self, start: usize) -> Token<'src> {
        // Get consecutive letters and "_" to create a token
        let mut len = 1;
        while let Some((_, ch)) = self.inner.peek()
            && (ch.is_ascii_alphanumeric() || *ch == '_')
        {
            len += 1;
            self.inner.next();
        }

        let lexeme = &self.source[start..start + len];
        let kind = match lexeme {
            "let" => TokenKind::Let,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "loop" => TokenKind::Loop,
            "break" => TokenKind::Break,
            "fun" => TokenKind::Fun,
            "return" => TokenKind::Return,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            _ => TokenKind::Ident,
        };

        Token::new(lexeme, kind, self.line)
    }
}

impl<'src> Iterator for Lexer<'src> {
    type Item = Token<'src>;

    fn next(&mut self) -> Option<Self::Item> {
        let (idx, ch) = match self.inner.next() {
            Some(next) => next,
            None if !self.at_eof => {
                self.at_eof = true;
                return Some(Token::new("", TokenKind::Eof, self.line));
            }
            None => return None,
        };

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
            '/' => match self.inner.next_if(|(_, c)| *c == '/') {
                Some(_) => {
                    while self.inner.next_if(|(_, c)| *c != '\n').is_some() {}
                    // FIXME: Recursive skipping can overflow the stack on many consecutive comment lines.
                    return self.next();
                }
                None => self.make_token_len(TokenKind::Slash, idx, 1),
            },
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
            ' ' | '\t' | '\r' => {
                // FIXME: Recursive skipping can overflow the stack on long runs of whitespace.
                return self.next();
            }
            '\n' => {
                self.line += 1;
                // FIXME: Recursive skipping can overflow the stack on long runs of newlines.
                return self.next();
            }
            '0'..='9' => self.integer(idx),
            'a'..='z' | 'A'..='Z' | '_' => self.ident_or_keyword(idx),
            _ => self.make_token_len(TokenKind::Error, idx, ch.len_utf8()),
        };

        Some(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::empty("", 1)]
    #[case::trailing_newlines("\n\n", 3)]
    #[case::comment_with_newline("// comment\n", 2)]
    fn yields_eof_once(#[case] source: &str, #[case] line: usize) {
        let mut lexer = Lexer::new(source);

        assert_eq!(lexer.next(), Some(Token::new("", TokenKind::Eof, line)));
        assert_eq!(lexer.next(), None);
        assert_eq!(lexer.next(), None);
    }

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
        assert_eq!(lexer.next(), Some(Token::new("", TokenKind::Eof, 1)));
        assert_eq!(lexer.next(), None);
    }

    #[rstest]
    #[case::zero("0")]
    #[case::single_digit("7")]
    #[case::multiple_digits("1234567890")]
    #[case::leading_zeros("00123")]
    fn yields_integer_tokens(#[case] source: &str) {
        let mut lexer = Lexer::new(source);

        assert_eq!(
            lexer.next(),
            Some(Token::new(source, TokenKind::Integer, 1))
        );
        assert_eq!(lexer.next(), Some(Token::new("", TokenKind::Eof, 1)));
        assert_eq!(lexer.next(), None);
    }

    #[rstest]
    #[case::letter("123a")]
    #[case::symbol("123;")]
    #[case::whitespace("123 ")]
    fn yields_integer_tokens_excludes_trailing_non_digit(#[case] source: &str) {
        let mut lexer = Lexer::new(source);

        assert_eq!(lexer.next(), Some(Token::new("123", TokenKind::Integer, 1)));
    }

    #[rstest]
    #[case::single_letter("a")]
    #[case::lowercase("count")]
    #[case::uppercase("COUNT")]
    #[case::mixed_case("itemCount")]
    #[case::underscore("_")]
    #[case::leading_underscore("_count")]
    #[case::embedded_underscore("item_count")]
    #[case::digits("item123")]
    #[case::keyword_prefix("return_value")]
    #[case::keyword_with_digit("if1")]
    #[case::keyword_different_case("Let")]
    fn yields_identifier_tokens(#[case] source: &str) {
        let mut lexer = Lexer::new(source);

        assert_eq!(lexer.next(), Some(Token::new(source, TokenKind::Ident, 1)));
        assert_eq!(lexer.next(), Some(Token::new("", TokenKind::Eof, 1)));
        assert_eq!(lexer.next(), None);
    }

    #[rstest]
    #[case::let_keyword("let", TokenKind::Let)]
    #[case::if_keyword("if", TokenKind::If)]
    #[case::else_keyword("else", TokenKind::Else)]
    #[case::loop_keyword("loop", TokenKind::Loop)]
    #[case::break_keyword("break", TokenKind::Break)]
    #[case::fun_keyword("fun", TokenKind::Fun)]
    #[case::return_keyword("return", TokenKind::Return)]
    #[case::true_literal("true", TokenKind::True)]
    #[case::false_literal("false", TokenKind::False)]
    fn yields_keyword_tokens(#[case] source: &str, #[case] kind: TokenKind) {
        let mut lexer = Lexer::new(source);

        assert_eq!(lexer.next(), Some(Token::new(source, kind, 1)));
        assert_eq!(lexer.next(), Some(Token::new("", TokenKind::Eof, 1)));
        assert_eq!(lexer.next(), None);
    }

    #[rstest]
    #[case::spaces(" ")]
    #[case::tabs("\t")]
    #[case::carriage_returns("\r")]
    #[case::mixed(" \t\r")]
    fn skips_whitespace(#[case] whitespace: &str) {
        let mut lexer = Lexer::new(whitespace);
        assert_eq!(lexer.next(), Some(Token::new("", TokenKind::Eof, 1)));
        assert_eq!(lexer.next(), None);
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
                Token::new("", TokenKind::Eof, 1),
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
                Token::new("", TokenKind::Eof, 3),
            ]
        );
    }

    #[test]
    fn yields_slash() {
        let mut lexer = Lexer::new("/");
        assert_eq!(lexer.next(), Some(Token::new("/", TokenKind::Slash, 1)));
        assert_eq!(lexer.next(), Some(Token::new("", TokenKind::Eof, 1)));
        assert_eq!(lexer.next(), None);
    }

    #[rstest]
    #[case::only_comment("// a comment")]
    #[case::empty_comment("//")]
    fn skips_line_comment_at_end_of_input(#[case] source: &str) {
        let mut lexer = Lexer::new(source);
        assert_eq!(lexer.next(), Some(Token::new("", TokenKind::Eof, 1)));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn comment_ends_at_newline_and_counts_the_line() {
        let tokens: Vec<_> = Lexer::new("+ // comment\n-").collect();
        assert_eq!(
            tokens,
            vec![
                Token::new("+", TokenKind::Plus, 1),
                Token::new("-", TokenKind::Minus, 2),
                Token::new("", TokenKind::Eof, 2),
            ]
        );
    }

    #[rstest]
    #[case::ascii("$")]
    #[case::non_ascii("é")]
    #[case::emoji("🦀")]
    fn yields_error_token_for_unexpected_character(#[case] source: &str) {
        let mut lexer = Lexer::new(source);
        assert_eq!(lexer.next(), Some(Token::new(source, TokenKind::Error, 1)));
        assert_eq!(lexer.next(), Some(Token::new("", TokenKind::Eof, 1)));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn yields_arithmetic_declaration_with_comment() {
        let source = "let count=(12+3)*2/-5; // initial count\n";
        let tokens: Vec<_> = Lexer::new(source).collect();

        assert_eq!(
            tokens,
            vec![
                Token::new("let", TokenKind::Let, 1),
                Token::new("count", TokenKind::Ident, 1),
                Token::new("=", TokenKind::Equal, 1),
                Token::new("(", TokenKind::LeftParen, 1),
                Token::new("12", TokenKind::Integer, 1),
                Token::new("+", TokenKind::Plus, 1),
                Token::new("3", TokenKind::Integer, 1),
                Token::new(")", TokenKind::RightParen, 1),
                Token::new("*", TokenKind::Star, 1),
                Token::new("2", TokenKind::Integer, 1),
                Token::new("/", TokenKind::Slash, 1),
                Token::new("-", TokenKind::Minus, 1),
                Token::new("5", TokenKind::Integer, 1),
                Token::new(";", TokenKind::Semicolon, 1),
                Token::new("", TokenKind::Eof, 2),
            ]
        );
    }

    #[test]
    fn yields_multiline_boolean_declaration() {
        let source = "let ready = (count >= 1)\n  & !false | true;";
        let tokens: Vec<_> = Lexer::new(source).collect();

        assert_eq!(
            tokens,
            vec![
                Token::new("let", TokenKind::Let, 1),
                Token::new("ready", TokenKind::Ident, 1),
                Token::new("=", TokenKind::Equal, 1),
                Token::new("(", TokenKind::LeftParen, 1),
                Token::new("count", TokenKind::Ident, 1),
                Token::new(">=", TokenKind::GreaterEqual, 1),
                Token::new("1", TokenKind::Integer, 1),
                Token::new(")", TokenKind::RightParen, 1),
                Token::new("&", TokenKind::Ampersand, 2),
                Token::new("!", TokenKind::Bang, 2),
                Token::new("false", TokenKind::False, 2),
                Token::new("|", TokenKind::Pipe, 2),
                Token::new("true", TokenKind::True, 2),
                Token::new(";", TokenKind::Semicolon, 2),
                Token::new("", TokenKind::Eof, 2),
            ]
        );
    }

    #[test]
    fn yields_function_definition_and_call() {
        let source = "fun add(a, b) {\n  return a+b;\n}\n\nadd(1,2);";
        let tokens: Vec<_> = Lexer::new(source).collect();

        assert_eq!(
            tokens,
            vec![
                Token::new("fun", TokenKind::Fun, 1),
                Token::new("add", TokenKind::Ident, 1),
                Token::new("(", TokenKind::LeftParen, 1),
                Token::new("a", TokenKind::Ident, 1),
                Token::new(",", TokenKind::Comma, 1),
                Token::new("b", TokenKind::Ident, 1),
                Token::new(")", TokenKind::RightParen, 1),
                Token::new("{", TokenKind::LeftBrace, 1),
                Token::new("return", TokenKind::Return, 2),
                Token::new("a", TokenKind::Ident, 2),
                Token::new("+", TokenKind::Plus, 2),
                Token::new("b", TokenKind::Ident, 2),
                Token::new(";", TokenKind::Semicolon, 2),
                Token::new("}", TokenKind::RightBrace, 3),
                Token::new("add", TokenKind::Ident, 5),
                Token::new("(", TokenKind::LeftParen, 5),
                Token::new("1", TokenKind::Integer, 5),
                Token::new(",", TokenKind::Comma, 5),
                Token::new("2", TokenKind::Integer, 5),
                Token::new(")", TokenKind::RightParen, 5),
                Token::new(";", TokenKind::Semicolon, 5),
                Token::new("", TokenKind::Eof, 5),
            ]
        );
    }

    #[test]
    fn yields_loop_with_conditional_break() {
        let source =
            "loop {\n  if (count!=0) {\n    count=count-1;\n  } else {\n    break;\n  }\n}";
        let tokens: Vec<_> = Lexer::new(source).collect();

        assert_eq!(
            tokens,
            vec![
                Token::new("loop", TokenKind::Loop, 1),
                Token::new("{", TokenKind::LeftBrace, 1),
                Token::new("if", TokenKind::If, 2),
                Token::new("(", TokenKind::LeftParen, 2),
                Token::new("count", TokenKind::Ident, 2),
                Token::new("!=", TokenKind::BangEqual, 2),
                Token::new("0", TokenKind::Integer, 2),
                Token::new(")", TokenKind::RightParen, 2),
                Token::new("{", TokenKind::LeftBrace, 2),
                Token::new("count", TokenKind::Ident, 3),
                Token::new("=", TokenKind::Equal, 3),
                Token::new("count", TokenKind::Ident, 3),
                Token::new("-", TokenKind::Minus, 3),
                Token::new("1", TokenKind::Integer, 3),
                Token::new(";", TokenKind::Semicolon, 3),
                Token::new("}", TokenKind::RightBrace, 4),
                Token::new("else", TokenKind::Else, 4),
                Token::new("{", TokenKind::LeftBrace, 4),
                Token::new("break", TokenKind::Break, 5),
                Token::new(";", TokenKind::Semicolon, 5),
                Token::new("}", TokenKind::RightBrace, 6),
                Token::new("}", TokenKind::RightBrace, 7),
                Token::new("", TokenKind::Eof, 7),
            ]
        );
    }
}
