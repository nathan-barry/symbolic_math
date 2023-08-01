pub mod operations;
pub mod eval;
pub mod simplify;
pub mod expansion;

use std::fmt::{self, Formatter, Display};
use crate::symbol::Symbol;

/// Represents a mathematical expression.
///
/// Expressions can be constants (floating point numbers), symbolic variables, or operations
/// (addition, subtraction, multiplication, division, exponentiation, negation). Each operation
/// can contain other expressions, allowing complex, nested expressions to be represented.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// A constant (floating point number).
    Const(f64),
    /// A symbolic variable.
    Symbol(Symbol),
    /// Addition of two expressions.
    Add(Box<Expr>, Box<Expr>),
    /// Subtraction of two expressions.
    Sub(Box<Expr>, Box<Expr>),
    /// Multiplication of two expressions.
    Mul(Box<Expr>, Box<Expr>),
    /// Division of two expressions.
    Div(Box<Expr>, Box<Expr>),
    /// Exponentiation of two expressions.
    Pow(Box<Expr>, Box<Expr>),
    /// Negation of an expression.
    Neg(Box<Expr>),
}

// Constructors
impl Expr {
    /// Constructs a new symbolic variable with the given name.
    ///
    /// # Examples
    ///
    /// ```
    /// use symbolic_math::expr::Expr;
    ///
    /// let x = Expr::new_var("x");
    /// ```
    pub fn new_var(str: &str) -> Expr {
        Expr::Symbol(Symbol::new(str))
    }

    /// Constructs a new constant value.
    ///
    /// # Examples
    ///
    /// ```
    /// use symbolic_math::expr::Expr;
    ///
    /// let two = Expr::new_val(2.0);
    /// ```
    pub fn new_val(val: f64) -> Expr {
        Expr::Const(val)
    }

}

// Borrows Data
impl Expr {
    /// If the expression is a symbolic variable, returns the symbol; otherwise, returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use symbolic_math::expr::Expr;
    /// use symbolic_math::symbol::Symbol;
    ///
    /// let x = Expr::new_var("x");
    /// assert_eq!(x.get_symbol().unwrap(), Symbol::new("x"));
    /// ```
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
            Expr::Mul(lhs, rhs) => {
                if let Expr::Const(c) = **lhs {
                    if let Expr::Symbol(_) = **rhs {
                        return write!(f, "{}{}", c, rhs);
                    }
                } else if let Expr::Const(c) = **rhs {
                    if let Expr::Symbol(_) = **lhs {
                        return write!(f, "{}{}", c, lhs);
                    }
                }
                write!(f, "({} * {})", lhs, rhs)
            }
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
