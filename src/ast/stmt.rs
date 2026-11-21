use crate::ast::expr::Expr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
    Let(LetStmt),
    Assign(AssignStmt),
    If,
    Loop,
    Break,
    Return,
    Expr(ExprStmt),
    Fun,
}

/// `let a, b = expr;` or `let a;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LetStmt {
    pub names: Vec<String>,
    pub init: Option<Expr>,
}

impl LetStmt {
    pub fn new(names: Vec<String>, init: Option<Expr>) -> Self {
        Self { names, init }
    }
}

impl From<LetStmt> for Stmt {
    fn from(stmt: LetStmt) -> Self {
        Self::Let(stmt)
    }
}

/// `name = expr;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssignStmt {
    pub name: String,
    pub value: Expr,
}

impl AssignStmt {
    pub fn new(name: String, value: Expr) -> Self {
        Self { name, value }
    }
}

impl From<AssignStmt> for Stmt {
    fn from(stmt: AssignStmt) -> Self {
        Self::Assign(stmt)
    }
}

/// An expression used as a statement, e.g. `foo(1, 2);`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExprStmt {
    pub expr: Expr,
}

impl ExprStmt {
    pub fn new(expr: Expr) -> Self {
        Self { expr }
    }
}

impl From<ExprStmt> for Stmt {
    fn from(stmt: ExprStmt) -> Self {
        Self::Expr(stmt)
    }
}
