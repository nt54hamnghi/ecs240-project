use crate::ast::expr::Expr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
    Let(LetStmt),
    Assign(AssignStmt),
    If(IfStmt),
    Loop(LoopStmt),
    Break,
    Return(ReturnStmt),
    Expr(ExprStmt),
    Fun(FunStmt),
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReturnStmt {
    pub value: Option<Expr>,
}

impl ReturnStmt {
    pub fn new(value: Option<Expr>) -> Self {
        Self { value }
    }
}

impl From<ReturnStmt> for Stmt {
    fn from(stmt: ReturnStmt) -> Self {
        Self::Return(stmt)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunStmt {
    pub name: String,
    pub params: Vec<String>,
    pub body: Block,
}

impl FunStmt {
    pub fn new(name: String, params: Vec<String>, body: Block) -> Self {
        Self { name, params, body }
    }
}

impl From<FunStmt> for Stmt {
    fn from(stmt: FunStmt) -> Self {
        Self::Fun(stmt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::expr::{BinaryExpr, IdentExpr, IntExpr, Op};

    fn ident(name: &str) -> Expr {
        IdentExpr::new(name.to_string()).into()
    }

    fn int(value: i64) -> Expr {
        IntExpr { value }.into()
    }

    // The `sum` example from the proposal (Section 3.1.2):
    //
    // fun sum(n) {
    //   let i = 0;
    //   loop {
    //     if (i >= n) {
    //       break;
    //     } else {
    //       i = i + 1;
    //     }
    //   }
    //   return i;
    // }
    #[test]
    fn builds_sum_example_from_proposal() {
        let if_stmt = IfStmt::new(
            BinaryExpr::new(ident("i"), Op::Ge, ident("n")).into(),
            Block::new(vec![Stmt::Break]),
            Some(Block::new(vec![
                AssignStmt::new(
                    "i".to_string(),
                    BinaryExpr::new(ident("i"), Op::Add, int(1)).into(),
                )
                .into(),
            ])),
        );

        let body = Block::new(vec![
            LetStmt::new(vec!["i".to_string()], Some(int(0))).into(),
            LoopStmt::new(Block::new(vec![if_stmt.into()])).into(),
            ReturnStmt::new(Some(ident("i"))).into(),
        ]);

        let sum: Stmt = FunStmt::new("sum".to_string(), vec!["n".to_string()], body).into();

        let Stmt::Fun(fun) = sum else {
            panic!("expected a function statement");
        };
        assert_eq!(fun.name, "sum");
        assert_eq!(fun.params, vec!["n".to_string()]);
        assert_eq!(fun.body.stmts.len(), 3);
        assert!(matches!(fun.body.stmts[1], Stmt::Loop(_)));
        assert_eq!(fun.body.stmts[2], ReturnStmt::new(Some(ident("i"))).into());
    }

    #[test]
    fn let_without_initializer_and_return_without_value() {
        // let a, b;
        let decl = LetStmt::new(vec!["a".to_string(), "b".to_string()], None);
        assert_eq!(decl.names.len(), 2);
        assert_eq!(decl.init, None);

        // return;
        assert_eq!(ReturnStmt::new(None).value, None);
    }
}
