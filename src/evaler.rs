use super::ast::*;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum EvalError {
    #[error("variable '{0}' is not defined")]
    NotDefinedVariable(String),
    #[error("cannot assign to '{0}'")]
    CannotAssign(String),
}

pub struct Evaler {
    vars: HashMap<Ident, i32>,
}

impl Evaler {
    pub fn new() -> Self {
        Self {
            vars: HashMap::from([(Ident::new("x".to_string()), 7)]),
        }
    }

    pub fn eval(&mut self, expr: &Expr) -> Result<i32, EvalError> {
        use Expr::*;
        match expr {
            Number(val) => Ok(val.eval()),
            Minus(val) => Ok(-self.eval(val)?),
            Add { left, right } => Ok(self.eval(left)? + self.eval(right)?),
            Sub { left, right } => Ok(self.eval(left)? - self.eval(right)?),
            Mul { left, right } => Ok(self.eval(left)? * self.eval(right)?),
            Div { left, right } => Ok(self.eval(left)? / self.eval(right)?),
            Var(ident) => match self.vars.get(ident) {
                Some(expr) => Ok(*expr),
                None => Err(EvalError::NotDefinedVariable(ident.to_string())),
            },
            Assign { left, right } => {
                if let Expr::Var(ident) = left.as_ref() {
                    let right_val = self.eval(right)?;
                    self.vars.insert(ident.as_ref().clone(), right_val);
                    Ok(right_val)
                } else {
                    Err(EvalError::CannotAssign(format!("{:?}", left)))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Expr {
        fn eval(&self) -> Result<i32, EvalError> {
            Evaler::new().eval(self)
        }
    }

    #[test]
    fn eval_number() {
        assert_eq!(Expr::Number(Box::new(Number::I32(7))).eval(), Ok(7));
    }

    #[test]
    fn eval_minus() {
        assert_eq!(
            Expr::Minus(Box::new(Number::I32(7).to_expr())).eval(),
            Ok(-7)
        );
    }

    #[test]
    fn eval_add() {
        assert_eq!(
            Expr::Add {
                left: Box::new(Number::I32(5).to_expr()),
                right: Box::new(Number::I32(2).to_expr())
            }
            .eval(),
            Ok(7)
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
            Ok(3)
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
            Ok(10)
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
            Ok(3)
        );
    }

    #[test]
    fn eval_var() {
        assert_eq!(Ident::new("x".to_string()).to_expr_var().eval(), Ok(7));
        assert_eq!(
            Ident::new("y".to_string()).to_expr_var().eval(),
            Err(EvalError::NotDefinedVariable("y".to_string()))
        );
    }
}
