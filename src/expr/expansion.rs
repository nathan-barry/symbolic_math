use crate::expr::Expr;

impl Expr {
    /// Expands the current expression to a possibly expanded form.
    ///
    /// The method expands mathematical expressions based on several
    /// algebraic rules.
    ///
    /// # Example
    ///
    /// ```
    /// use symbolic_math::expr::Expr;
    ///
    /// let x = Expr::new_var("x");
    /// let y = Expr::new_var("y");
    /// let res = (x.clone() + y.clone()) * Expr::new_val(2.0);
    /// assert_eq!(res.expand(), x * Expr::new_val(2.0) + y * Expr::new_val(2.0));
    /// ```
    pub fn expand(&self) -> Expr {
        match self {
            Expr::Mul(lhs, rhs) => {
                let lhs = lhs.expand();
                let rhs = rhs.expand();
                match (&lhs, &rhs) {
                    // (a + b) * c -> a*c + b*c
                    (Expr::Add(a, b), c) | (c, Expr::Add(a, b)) => 
                        Expr::Add(Box::new(Expr::Mul(a.clone(), Box::new(c.clone()))),
                                  Box::new(Expr::Mul(b.clone(), Box::new(c.clone())))).expand(),
                    // c * (a - b) -> c*a - c*b
                    (Expr::Sub(a, b), c) | (c, Expr::Sub(a, b)) => 
                        Expr::Sub(Box::new(Expr::Mul(Box::new(c.clone()), a.clone())),
                                  Box::new(Expr::Mul(Box::new(c.clone()), b.clone()))).expand(),
                    _ => Expr::Mul(Box::new(lhs), Box::new(rhs)),
                }
            },
            Expr::Add(lhs, rhs) => Expr::Add(Box::new(lhs.expand()), Box::new(rhs.expand())),
            Expr::Sub(lhs, rhs) => Expr::Sub(Box::new(lhs.expand()), Box::new(rhs.expand())),
            Expr::Div(lhs, rhs) => Expr::Div(Box::new(lhs.expand()), Box::new(rhs.expand())),
            Expr::Pow(lhs, rhs) => Expr::Pow(Box::new(lhs.expand()), Box::new(rhs.expand())),
            _ => self.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expansion() {
        // Testing (a + b) * c -> a*c + b*c
        let x = Expr::new_var("x");
        let y = Expr::new_var("y");
        let res = (x.clone() + y.clone()) * Expr::new_val(2.0);
        assert_eq!(res.expand(), x.clone() * Expr::new_val(2.0) + y.clone() * Expr::new_val(2.0));

        // Testing c * (a - b) -> c*a - c*b
        let a = Expr::new_val(3.0);
        let b = Expr::new_val(2.0);
        let c = Expr::new_var("c");
        let res = c.clone() * (a.clone() - b.clone());
        assert_eq!(res.expand(), c.clone() * a.clone() - c.clone() * b.clone());
    }
}
