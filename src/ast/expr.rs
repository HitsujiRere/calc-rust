use super::*;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(Box<Number>),
    Minus(Box<Expr>),
    Add { left: Box<Expr>, right: Box<Expr> },
    Sub { left: Box<Expr>, right: Box<Expr> },
    Mul { left: Box<Expr>, right: Box<Expr> },
    Div { left: Box<Expr>, right: Box<Expr> },
}

impl Eval for Expr {
    fn eval(&self) -> i32 {
        use Expr::*;
        match self {
            Number(val) => val.eval(),
            Minus(val) => -val.eval(),
            Add { left, right } => left.eval() + right.eval(),
            Sub { left, right } => left.eval() - right.eval(),
            Mul { left, right } => left.eval() * right.eval(),
            Div { left, right } => left.eval() / right.eval(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_number() {
        assert_eq!(Expr::Number(Box::new(Number::I32(7))).eval(), 7);
    }

    #[test]
    fn eval_minus() {
        assert_eq!(Expr::Minus(Box::new(Number::I32(7).to_expr())).eval(), -7);
    }

    #[test]
    fn eval_add() {
        assert_eq!(
            Expr::Add {
                left: Box::new(Number::I32(5).to_expr()),
                right: Box::new(Number::I32(2).to_expr())
            }
            .eval(),
            7
        );
    }

    #[test]
    fn eval_sub() {
        assert_eq!(
            Expr::Sub {
                left: Box::new(Number::I32(5).to_expr()),
                right: Box::new(Number::I32(2).to_expr())
            }
            .eval(),
            3
        );
    }

    #[test]
    fn eval_mul() {
        assert_eq!(
            Expr::Mul {
                left: Box::new(Number::I32(2).to_expr()),
                right: Box::new(Number::I32(5).to_expr())
            }
            .eval(),
            10
        );
    }

    #[test]
    fn eval_div() {
        assert_eq!(
            Expr::Div {
                left: Box::new(Number::I32(6).to_expr()),
                right: Box::new(Number::I32(2).to_expr())
            }
            .eval(),
            3
        );
    }

    #[test]
    fn number_to_expr() {
        assert_eq!(
            Number::I32(7).to_expr(),
            Expr::Number(Box::new(Number::I32(7)))
        );
    }
}
