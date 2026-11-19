#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
    Let,
    Assign,
    If,
    Loop,
    Break,
    Return,
    Expr,
    Fun,
}

// TODO: define individual Stmt type, constructors, and From conversion
