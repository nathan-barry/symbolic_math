use crate::expr::Expr;

impl Expr {
    /// Simplifies the current expression to a possibly simpler form.
    ///
    /// The method simplifies the mathematical expressions based on several
    /// algebraic rules.
    ///
    /// # Example
    ///
    /// ```
    /// let x1 = Expr::new_var("x");
    /// let x2 = Expr::new_var("x");
    /// let res = x1 + x2;
    /// assert_eq!(res.simplify(), Expr::new_val(2.0) * Expr::new_var("x"));
    /// ```
    pub fn simplify(&self) -> Expr {
        match self {
            Expr::Add(lhs, rhs) => {
                let lhs = lhs.simplify();
                let rhs = rhs.simplify();
                match (&lhs, &rhs) {
                    // lhs == rhs, return 2 * lhs
                    (lhs, rhs) if *lhs == *rhs =>
                        Expr::Mul(Box::new(Expr::new_val(2.0)), Box::new(lhs.clone())),
                    // cx + x, return (c+1)x 
                    (Expr::Mul(c, inside), out)
                        | (Expr::Mul(inside, c), out)
                        | (out, Expr::Mul(inside, c))
                        | (out, Expr::Mul(c, inside))
                        if ((**inside == *out ) && c.is_const()) =>
                        Expr::Mul(Box::new(Expr::new_val(c.get_const() + 1.0)), Box::new(out.clone())),
                    // Both constants, return mul
                    (Expr::Const(c1), Expr::Const(c2)) =>
                        Expr::new_val(c1 + c2),
                    // Constant == 0, return Expr unchanged
                    (Expr::Const(c), x)
                        | (x, Expr::Const(c))
                        if *c == 0.0 => x.clone(),
                    // Else
                    _ => Expr::Add(Box::new(lhs), Box::new(rhs)),
                }
            },
            Expr::Sub(lhs, rhs) => {
                let lhs = lhs.simplify();
                let rhs = rhs.simplify();
                match (&lhs, &rhs) {
                    // Both constants, return diff
                    (Expr::Const(c1), Expr::Const(c2)) => Expr::new_val(c1 - c2),
                    // Constant == 0, return Expr unchanged
                    (Expr::Const(c), x)
                        | (x, Expr::Const(c))
                        if *c == 0.0 => x.clone(),
                    // Else
                    _ => Expr::Sub(Box::new(lhs), Box::new(rhs)),
                }
            },
            Expr::Mul(lhs, rhs) => {
                let lhs = lhs.simplify();
                let rhs = rhs.simplify();
                match (&lhs, &rhs) {
                    // lhs == rhs, return lhs^2
                    (lhs, rhs) if *lhs == *rhs =>
                        Expr::Pow(Box::new(lhs.clone()), Box::new(Expr::new_val(2.0))),
                    // x^a * x^b, return x^(a+b)
                    (Expr::Pow(base1, a), Expr::Pow(base2, b)) if *base1 == *base2 =>
                        Expr::Pow(base1.clone(), Box::new(Expr::Add(a.clone(), b.clone()))),
                    // Both constants, return mul
                    (Expr::Const(c1), Expr::Const(c2)) => Expr::new_val(c1 * c2),
                    // Constant == 1, return Expr unchanged
                    (x, Expr::Const(c))
                        | (Expr::Const(c), x)
                        if *c == 1.0 => x.clone(),
                    // Constant == 0, return 0
                    (Expr::Const(c), _)
                        | (_, Expr::Const(c))
                        if *c == 0.0 => Expr::Const(0.0),
                    // Constant == -1, return Neg
                    (Expr::Const(c), x)
                        | (x, Expr::Const(c))
                        if *c == -1.0 => Expr::Neg(Box::new(x.clone())),
                    // Else
                    _ => Expr::Mul(Box::new(lhs), Box::new(rhs)),
                }
            },
            Expr::Div(lhs, rhs) => {
                let lhs = lhs.simplify();
                let rhs = rhs.simplify();
                match (&lhs, &rhs) {
                    // Both constants, return div
                    (Expr::Const(c1), Expr::Const(c2)) => Expr::new_val(c1 / c2),
                    // Symbol, constant == 1, return symbol
                    (x, Expr::Const(c))
                        | (Expr::Const(c), x)
                        if *c == 1.0 => x.clone(),
                    // 0 divided by x, return 0
                    (Expr::Const(c), _) if *c == 0.0 => Expr::Const(0.0),
                    // Else
                    _ => Expr::Div(Box::new(lhs), Box::new(rhs)),
                }
            },
            Expr::Pow(lhs, rhs) => {
                let lhs = lhs.simplify();
                let rhs = rhs.simplify();
                match (&lhs, &rhs) {
                    // (x^a)^b, returns x^(a*b)
                    (Expr::Pow(base, p1), p2) =>
                        Expr::Pow(
                            base.clone(),
                            Box::new(Expr::Mul(p1.clone(), Box::new(p2.clone())))
                        ),
                    // x^1, returns x
                    (x, Expr::Const(c)) if *c == 1.0 => x.clone(),
                    // x^0, returns 1
                    (_, Expr::Const(c)) if *c == 0.0 => Expr::Const(1.0), // TODO: Only if x != 0
                    // 1^x, returns 1
                    (Expr::Const(c), _) if *c == 1.0 => Expr::Const(1.0),
                    // Else
                    _ => Expr::Pow(Box::new(lhs), Box::new(rhs))
                }
            },
            _ => self.clone()
        }
    }

    /// Checks if the current expression is a constant.
    ///
    /// Returns `true` if the current instance of `Expr` is a `Const` variant, and
    /// `false` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// let expr = Expr::new_val(2.0);
    /// assert_eq!(expr.is_const(), true);
    /// ```
    fn is_const(&self) -> bool {
        if let Expr::Const(_) = self { true } else { false }
    }

    /// Returns the `f64` value inside the `Const` variant of `Expr`.
    ///
    /// # Panics
    ///
    /// This function will panic if called on a non-`Const` `Expr`.
    ///
    /// # Example
    ///
    /// ```
    /// let expr = Expr::new_val(2.0);
    /// assert_eq!(expr.get_const(), 2.0);
    /// ```
    fn get_const(&self) -> f64 {
        match self {
            Expr::Const(c) => *c,
            _ => panic!("Cannot call get_const on non-const Expr")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_const() {
        let c1 = Expr::new_val(2.0);
        let c2 = Expr::new_val(4.0);
        let res = c1 + c2;

        assert_eq!(res.simplify(), Expr::new_val(6.0));
    }

    #[test]
    fn sub_const() {
        let c1 = Expr::new_val(2.0);
        let c2 = Expr::new_val(4.0);
        let res = c1 - c2;

        assert_eq!(res.simplify(), Expr::new_val(-2.0));
    }

    #[test]
    fn mul_const() {
        let c1 = Expr::new_val(2.0);
        let c2 = Expr::new_val(4.0);
        let res = c1 * c2;

        assert_eq!(res.simplify(), Expr::new_val(8.0));
    }

    #[test]
    fn div_const() {
        let c1 = Expr::new_val(2.0);
        let c2 = Expr::new_val(4.0);
        let res = c1 / c2;

        assert_eq!(res.simplify(), Expr::new_val(0.5));
    }

    #[test]
    fn add_like_terms() {
        let x1 = Expr::new_var("x");
        let x2 = Expr::new_var("x");
        let res = x1 + x2;

        assert_eq!(res.simplify(), Expr::new_val(2.0) * Expr::new_var("x"));
    }
}
