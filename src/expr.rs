use std::fmt::{self, Formatter, Display};
use crate::symbol::Symbol;

/// The 'Expr' enum represents a mathematical expression
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Const(f64),
    Symbol(Symbol),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}

// Constructors
impl Expr {
    pub fn new_var(str: &str) -> Expr {
        Expr::Symbol(Symbol::new(str))
    }

    pub fn new_val(val: f64) -> Expr {
        Expr::Const(val)
    }
}

// Borrows Data
impl Expr {
    pub fn get_symbol(&self) -> Option<Symbol> {
        match self {
            Expr::Symbol(s) => Some(s.clone()),
            _ => None
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Expr::Const(c) => write!(f, "{}", c),
            Expr::Symbol(s) => write!(f, "{}", s.name),
            Expr::Add(lhs, rhs) => write!(f, "({} + {})", lhs, rhs),
            Expr::Sub(lhs, rhs) => write!(f, "({} - {})", lhs, rhs),
            Expr::Mul(lhs, rhs) => write!(f, "({} * {})", lhs, rhs),
            Expr::Div(lhs, rhs) => write!(f, "({} / {})", lhs, rhs),
            Expr::Pow(lhs, rhs) => write!(f, "({} ^ {})", lhs, rhs),
            Expr::Neg(expr) => write!(f, "-{}", expr),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_const() {
        let lhs = Expr::Const(2.0);
        let rhs = Expr::Const(4.0);
        assert_eq!(Expr::Add(Box::new(lhs.clone()), Box::new(rhs.clone())), lhs + rhs);
    }
}
