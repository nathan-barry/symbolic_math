use std::ops;
use crate::symbol::Symbol;

/// The 'Expr' enum represents a mathematical expression
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Constant(f64),
    Symbol(Symbol),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}

impl Expr {
    pub fn new_var(symbol: Symbol) -> Expr {
        Expr::Symbol(symbol)
    }
}

impl ops::Add for Expr {
    type Output = Expr;

    fn add(self, rhs: Expr) -> Expr {
        Expr::Add(Box::new(self), Box::new(rhs))
    }
}

impl ops::Sub for Expr {
    type Output = Expr;

    fn sub(self, rhs: Expr) -> Expr {
        Expr::Sub(Box::new(self), Box::new(rhs))
    }
}

impl ops::Mul for Expr {
    type Output = Expr;

    fn mul(self, rhs: Expr) -> Expr {
        Expr::Mul(Box::new(self), Box::new(rhs))
    }
}

impl ops::Div for Expr {
    type Output = Expr;

    fn div(self, rhs: Expr) -> Expr {
        Expr::Div(Box::new(self), Box::new(rhs))
    }
}

impl ops::BitXor for Expr {
    type Output = Expr;

    fn bitxor(self, rhs: Expr) -> Expr {
        Expr::Pow(Box::new(self), Box::new(rhs))
    }
}

impl ops::Neg for Expr {
    type Output = Expr;

    fn neg(self) -> Expr {
        Expr::Neg(Box::new(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_const() {
        let lhs = Expr::Constant(2.0);
        let rhs = Expr::Constant(4.0);
        assert_eq!(Expr::Add(Box::new(lhs.clone()), Box::new(rhs.clone())), lhs + rhs);
    }
}
