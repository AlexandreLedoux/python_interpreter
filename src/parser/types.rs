pub enum Operator {
    Add,
    Sub
}

pub enum Comparator {
    Eq
}

pub enum Expr {
    Literal(Literal),
    Binary {
        left: Box<Expr>,
        op: Operator,
        right: Box<Expr>
    }
}

pub enum Literal {
    Str(String),
    Int(u32)
}
