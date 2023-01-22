use super::ast::*;

pub struct Evaler {}

impl Evaler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn eval(&self, expr: &Expr) -> i32 {
        use Expr::*;
        match expr {
            Number(val) => val.eval(),
            Minus(val) => -self.eval(val),
            Add { left, right } => self.eval(left) + self.eval(right),
            Sub { left, right } => self.eval(left) - self.eval(right),
            Mul { left, right } => self.eval(left) * self.eval(right),
            Div { left, right } => self.eval(left) / self.eval(right),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Expr {
        fn eval(&self) -> i32 {
            Evaler::new().eval(self)
        }
    }

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
}
