use super::*;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(Box<Number>),
    Minus(Box<Expr>),
    Add { left: Box<Expr>, right: Box<Expr> },
    Sub { left: Box<Expr>, right: Box<Expr> },
    Mul { left: Box<Expr>, right: Box<Expr> },
    Div { left: Box<Expr>, right: Box<Expr> },
    Var(Box<Ident>),
}
