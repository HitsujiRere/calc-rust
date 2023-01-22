use super::*;

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Ident(String);

impl Ident {
    pub fn new(str: String) -> Self {
        Ident(str)
    }

    pub fn to_expr_var(self) -> Expr {
        Expr::Var(Box::new(self))
    }
}

impl ToString for Ident {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ident_to_expr_var() {
        assert_eq!(
            Ident::new("abc".to_string()).to_expr_var(),
            Expr::Var(Box::new(Ident::new("abc".to_string())))
        );
    }
}
