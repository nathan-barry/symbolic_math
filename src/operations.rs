use std::ops;
use crate::expr::Expr;

// Takes ownership
impl Expr {
    /// Raises an `Expr` instance to the power of another, creating a new `Expr::Pow` variant.
    ///
    /// This method consumes the original `Expr` instances, and produces a new one that
    /// represents the mathematical operation of exponentiation.
    ///
    /// # Arguments
    ///
    /// * `self` - The base of the exponentiation.
    /// * `expr` - The exponent in the exponentiation.
    ///
    /// # Examples
    ///
    /// ```
    /// let a = Expr::new_val(2.0);
    /// let b = Expr::new_val(3.0);
    /// let result = a.pow(b);
    /// ```
    ///
    /// Note: This function consumes the `Expr` instances that it operates on.
    pub fn pow(self, expr: Expr) -> Expr {
        Expr::Pow(Box::new(self), Box::new(expr))
    }
}

// Add Overload Operation implementations
impl ops::Add for Expr {
    type Output = Expr;

    fn add(self, rhs: Expr) -> Expr {
        Expr::Add(Box::new(self), Box::new(rhs))
    }
}

impl ops::Add<f64> for Expr {
    type Output = Expr;

    fn add(self, rhs: f64) -> Expr {
        Expr::Add(Box::new(self), Box::new(Expr::new_val(rhs)))
    }
}

impl ops::Add<Expr> for f64 {
    type Output = Expr;

    fn add(self, rhs: Expr) -> Expr {
        Expr::Add(Box::new(Expr::new_val(self)), Box::new(rhs))
    }
}

// Sub Overload Operation implementations
impl ops::Sub for Expr {
    type Output = Expr;

    fn sub(self, rhs: Expr) -> Expr {
        Expr::Sub(Box::new(self), Box::new(rhs))
    }
}

impl ops::Sub<f64> for Expr {
    type Output = Expr;

    fn sub(self, rhs: f64) -> Expr {
        Expr::Sub(Box::new(self), Box::new(Expr::new_val(rhs)))
    }
}

impl ops::Sub<Expr> for f64 {
    type Output = Expr;

    fn sub(self, rhs: Expr) -> Expr {
        Expr::Sub(Box::new(Expr::new_val(self)), Box::new(rhs))
    }
}

// Mul Overload Operation implementations
impl ops::Mul for Expr {
    type Output = Expr;

    fn mul(self, rhs: Expr) -> Expr {
        Expr::Mul(Box::new(self), Box::new(rhs))
    }
}

impl ops::Mul<f64> for Expr {
    type Output = Expr;

    fn mul(self, rhs: f64) -> Expr {
        Expr::Mul(Box::new(self), Box::new(Expr::new_val(rhs)))
    }
}

impl ops::Mul<Expr> for f64 {
    type Output = Expr;

    fn mul(self, rhs: Expr) -> Expr {
        Expr::Mul(Box::new(Expr::new_val(self)), Box::new(rhs))
    }
}

// Div Overload Operation implementations
impl ops::Div for Expr {
    type Output = Expr;

    fn div(self, rhs: Expr) -> Expr {
        Expr::Div(Box::new(self), Box::new(rhs))
    }
}

impl ops::Div<f64> for Expr {
    type Output = Expr;

    fn div(self, rhs: f64) -> Expr {
        Expr::Div(Box::new(self), Box::new(Expr::new_val(rhs)))
    }
}

impl ops::Div<Expr> for f64 {
    type Output = Expr;

    fn div(self, rhs: Expr) -> Expr {
        Expr::Div(Box::new(Expr::new_val(self)), Box::new(rhs))
    }
}

// Neg Overload Operation implementations
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
    fn test_add() {
        let x = Expr::new_var("x");
        assert_eq!(x.clone() + Expr::new_val(2.0), x.clone() + 2.0);
        assert_eq!(Expr::new_val(2.0) + x.clone(), 2.0 + x.clone());
    }

    #[test]
    fn test_sub() {
        let x = Expr::new_var("x");
        assert_eq!(x.clone() - Expr::new_val(2.0), x.clone() - 2.0);
        assert_eq!(Expr::new_val(2.0) - x.clone(), 2.0 - x.clone());
    }

    #[test]
    fn test_mul() {
        let x = Expr::new_var("x");
        assert_eq!(x.clone() * Expr::new_val(2.0), x.clone() * 2.0);
        assert_eq!(Expr::new_val(2.0) * x.clone(), 2.0 * x.clone());
    }

    #[test]
    fn test_div() {
        let x = Expr::new_var("x");
        assert_eq!(x.clone() / Expr::new_val(2.0), x.clone() / 2.0);
        assert_eq!(Expr::new_val(2.0) / x.clone(), 2.0 / x.clone());
    }
}
