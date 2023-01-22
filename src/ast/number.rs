use super::*;

#[derive(Debug, PartialEq)]
pub enum Number {
    I32(i32),
}

impl Number {
    pub fn to_expr(self) -> Expr {
        Expr::Number(Box::new(self))
    }

    pub fn eval(&self) -> i32 {
        use Number::*;
        match self {
            I32(val) => *val,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_to_expr() {
        assert_eq!(
            Number::I32(7).to_expr(),
            Expr::Number(Box::new(Number::I32(7)))
        );
    }

    #[test]
    fn eval_number() {
        assert_eq!(Number::I32(7).eval(), 7);
    }
}
