pub enum Stmt {
    Let {               // Let x, y, z
        names: Vec<String>,
    },
    
    LetInit {   // let x = 0, let x = y, let x;
        name: String,               // A variable
        value: Expr,       // Expression
    },

    Assign {    // x = 100, x = y
        name: String,               // A variab;e
        value: Expr                 // Must be an expression
    },

    If {
        condition: Expr,
        body: Vec<Stmt>,
        else_body: Option<Vec<Stmt>>,
    },

    Loop {
        body: Vec<Stmt>,            // Multiple statements
    },

    Break,
    ExprStmt{
        expression: Expr,
    },

    Return {
        value: Option<Expr>,
    },

    Fun {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },
}