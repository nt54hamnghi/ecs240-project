// Parse variable declarations
pub fn parse_let_stmt(&mut self) -> Result<Stmt, ParseError> {
    todo!()
}

// Parse variable assignments
pub fn parse_assign_stmt(&mut self) -> Result<Stmt, ParseError> {
    todo!()
}

// Parse If-else statements
pub fn parse_if_stmt(&mut self) -> Result<Stmt, ParseError> {
    todo!()
}

// Parse loop statement
pub fn parse_loop_stmt(&mut self) -> Result<Stmt, ParseError> {
    todo!()
}

// Parse break statement
pub fn parse_break_stmt(&mut self) -> Result<Stmt, ParseError> {
    todo!()
}

// Parse return statement
pub fn parse_return_stmt(&mut self) -> Result<Stmt, ParseError> {
    todo!()
}

// Parse a group of statements inside { } (block)
pub fn parse_block(&mut self) -> Result <Vec<Stmt>, ParseError> {
    todo!()
}

// Parse expression statements (those followed by ;)
pub fn parse_expr_stmt(&mut self) -> Result <Stmt, ParseError> {
    todo!()
}

// Parse the whole program
pub fn parse_program(&mut self) -> Result<Vec<Stmt>, ParseError> {
    todo!()
}

// Determine which kind of statement to parse
pub fn parse_block_stmt(&mut self) -> Result<Stmt, ParseError> {
    todo!()
}

// Parse a function declaration
pub fn parse_fun_stmt(&mut self) -> Result<Stmt, ParseError> {
    todo!()
}

// Parse function parameters
// Returns Vec<String> instead of Stmt because parameters are simply a collection of identifiers, not statements
pub fn parse_params(&mut self) -> Result<Vec<String>, ParseError> {
    todo!()
}

// Choose between block-stmt and func-stmt
pub fn parse_stmt(&mut self) -> Result<Stmt, ParseError> {
    todo!()
}


