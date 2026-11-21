use crate::ast::expr::Expr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
    Let(LetStmt),
    Assign(AssignStmt),
    If(IfStmt),
    Loop(LoopStmt),
    Break,
    Return,
    Expr(ExprStmt),
    Fun,
}

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

impl Block {
    pub fn new(stmts: Vec<Stmt>) -> Self {
        Self { stmts }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfStmt {
    pub cond: Expr,
    pub then_block: Block,
    pub else_block: Option<Block>,
}

impl IfStmt {
    pub fn new(cond: Expr, then_block: Block, else_block: Option<Block>) -> Self {
        Self {
            cond,
            then_block,
            else_block,
        }
    }
}

impl From<IfStmt> for Stmt {
    fn from(stmt: IfStmt) -> Self {
        Self::If(stmt)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoopStmt {
    pub body: Block,
}

impl LoopStmt {
    pub fn new(body: Block) -> Self {
        Self { body }
    }
}

impl From<LoopStmt> for Stmt {
    fn from(stmt: LoopStmt) -> Self {
        Self::Loop(stmt)
    }
}
