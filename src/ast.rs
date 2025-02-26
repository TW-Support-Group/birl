#[derive(Debug)]
pub enum Node {
    Program(Vec<Node>),
    Stmt(Stmt),
    Expr(Expr),
}

impl Node {
    pub fn stmts(&self) -> Option<&Vec<Node>> {
        if let Node::Program(stmts) = self {
            return Some(stmts);
        }
        None
    }
}

#[derive(Debug)]
pub enum Stmt {
    PrintStmt { format: String },
    ReturnStmt { value: Box<Node> },
}

impl Stmt {
    pub fn print(format: &str) -> Node {
        Node::Stmt(Stmt::PrintStmt {
            format: format.to_string(),
        })
    }

    pub fn return_(value: Box<Node>) -> Node {
        Node::Stmt(Stmt::ReturnStmt { value })
    }
}

#[derive(Debug)]
pub enum Expr {
    String(String),
    Number(f64),
}
