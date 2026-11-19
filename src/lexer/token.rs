#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Token<'src> {
    pub lexeme: &'src str,
    pub kind: TokenKind,
    pub line: usize,
}

impl<'src> Token<'src> {
    /// Create a token with its source lexeme, kind, and one-based line number.
    ///
    /// # Panics
    ///
    /// Panics if `line` is 0.
    pub const fn new(lexeme: &'src str, kind: TokenKind, line: usize) -> Self {
        assert!(line > 0, "line numbers start at 1");
        Self { lexeme, kind, line }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
    // Punctuation.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Semicolon,

    // Arithmetic operators.
    Plus,
    Minus,
    Star,
    Slash,

    // Boolean operators: !, &, and |.
    Bang,
    Ampersand,
    Pipe,

    // Assignment and comparison operators.
    Equal,
    EqualEqual,
    BangEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Keywords.
    Let,
    If,
    Else,
    Loop,
    Break,
    Fun,
    Return,

    // Identifiers and literals
    /// An identifier (`IDENT`).
    Ident,
    Integer,
    True,
    False,

    /// A character the lexer does not recognize; the lexeme is that character.
    Error,

    /// End of input.
    Eof,
}
