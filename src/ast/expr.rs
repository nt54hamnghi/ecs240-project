#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    // Integer arithmetic operators
    Add,
    Sub,
    Mul,
    Div,
    Neg,

    // Integer comparison operators
    Eq, // ==
    Ne, // !=
    Gt, // >
    Ge, // >=
    Lt, // <
    Le, // <=

    // Boolean operators
    Not,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Integer(IntExpr),
    Boolean(BoolExpr),
    Ident(IdentExpr),
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Call(CallExpr),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntExpr {
    pub value: i64,
}

impl From<IntExpr> for Expr {
    fn from(expr: IntExpr) -> Self {
        Self::Integer(expr)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoolExpr {
    pub value: bool,
}

impl From<BoolExpr> for Expr {
    fn from(expr: BoolExpr) -> Self {
        Self::Boolean(expr)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentExpr {
    pub name: String,
}

impl IdentExpr {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl From<IdentExpr> for Expr {
    fn from(expr: IdentExpr) -> Self {
        Self::Ident(expr)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnaryExpr {
    pub op: Op,
    pub operand: Box<Expr>,
}

impl UnaryExpr {
    pub fn new(op: Op, operand: Expr) -> Self {
        Self {
            op,
            operand: Box::new(operand),
        }
    }
}

impl From<UnaryExpr> for Expr {
    fn from(expr: UnaryExpr) -> Self {
        Self::Unary(expr)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub op: Op,
    pub right: Box<Expr>,
}

impl BinaryExpr {
    pub fn new(left: Expr, op: Op, right: Expr) -> Self {
        Self {
            left: Box::new(left),
            op,
            right: Box::new(right),
        }
    }
}

impl From<BinaryExpr> for Expr {
    fn from(expr: BinaryExpr) -> Self {
        Self::Binary(expr)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallExpr {
    pub callee: String,
    pub args: Vec<Expr>,
}

impl CallExpr {
    pub fn new(callee: String, args: Vec<Expr>) -> Self {
        Self { callee, args }
    }
}

impl From<CallExpr> for Expr {
    fn from(expr: CallExpr) -> Self {
        Self::Call(expr)
    }
}
