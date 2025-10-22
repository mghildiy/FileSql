#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Select(SelectStatement)
}

#[derive(Debug, PartialEq, Clone)]
pub struct SelectStatement {
    pub columns: Vec<SelectItem>,
    pub from: Option<FromClause>,
    pub where_clause: Option<Expr>,
    pub group_by: Vec<Expr>,
    pub order_by: Vec<OrderByItem>
}

#[derive(Debug, PartialEq, Clone)]
pub struct OrderByItem {
    pub expr: Expr,
    pub asc: bool
}

#[derive(Debug, PartialEq, Clone)]
pub enum SelectItem {
    Wildcard,
    Column(String),
    Aggregate {
        func: AggregateFunc,
        // If its a Count aggregate function, then None means it's a case of Count(*)
        // using box type so that size of enum is small
        expr: Option<Box<Expr>>
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum AggregateFunc {
    Sum,
    Avg,
    Count,
    Min,
    Max
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct FromClause {
    pub source: String
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Column(String),
    Literal(Value),
    BinaryOp {
        left: Box<Expr>,
        operator: BinaryOperator,
        right: Box<Expr>,
    },
    UnaryOp {
        operator: UnaryOperator,
        expr: Box<Expr>,
    }
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub enum BinaryOperator {
    // Comparison
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEquals,
    LessThanOrEquals,

    // Logical
    And,
    Or,

    // Arithmetic
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub enum UnaryOperator {
    Not,
    Minus,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
}