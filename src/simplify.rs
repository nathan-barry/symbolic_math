use crate::expr::Expr;

impl Expr {
    pub fn simplify(&self) -> Expr {
        match self {
            Expr::Add(lhs, rhs) => {
                let lhs = lhs.simplify();
                let rhs = rhs.simplify();
                match (&lhs, &rhs) {
                    (Expr::Const(c1), Expr::Const(c2)) => Expr::new_val(c1 + c2),
                    (Expr::Const(c), x) | (x, Expr::Const(c)) if *c == 0.0 => x.clone(),
                    _ => Expr::Add(Box::new(lhs), Box::new(rhs)),
                }
            },
            Expr::Sub(lhs, rhs) => {
                let lhs = lhs.simplify();
                let rhs = rhs.simplify();
                match (&lhs, &rhs) {
                    (Expr::Const(c1), Expr::Const(c2)) => Expr::new_val(c1 - c2),
                    (Expr::Const(c), x) | (x, Expr::Const(c)) if *c == 0.0 => x.clone(),
                    _ => Expr::Sub(Box::new(lhs), Box::new(rhs)),
                }
            },
            Expr::Mul(lhs, rhs) => {
                let lhs = lhs.simplify();
                let rhs = rhs.simplify();
                match (&lhs, &rhs) {
                    (Expr::Const(c1), Expr::Const(c2)) => Expr::new_val(c1 * c2),
                    (Expr::Symbol(s), Expr::Const(c)) | (Expr::Const(c), Expr::Symbol(s)) if *c == 1.0 => Expr::new_var(&s.name),
                    (Expr::Const(c), _) | (_, Expr::Const(c)) if *c == 0.0 => Expr::Const(0.0),
                    (Expr::Const(c), x) | (x, Expr::Const(c)) if *c == -1.0 => Expr::Neg(Box::new(x.clone())),
                    _ => Expr::Mul(Box::new(lhs), Box::new(rhs)),
                }
            },
            Expr::Div(lhs, rhs) => {
                let lhs = lhs.simplify();
                let rhs = rhs.simplify();
                match (&lhs, &rhs) {
                    (Expr::Const(c1), Expr::Const(c2)) => Expr::new_val(c1 / c2),
                    (Expr::Symbol(s), Expr::Const(c)) | (Expr::Const(c), Expr::Symbol(s)) if *c == 1.0 => Expr::new_var(&s.name),
                    (Expr::Const(c), _) if *c == 0.0 => Expr::Const(0.0),
                    _ => Expr::Div(Box::new(lhs), Box::new(rhs)),
                }
            },
            Expr::Pow(lhs, rhs) => {
                let lhs = lhs.simplify();
                let rhs = rhs.simplify();
                match (&lhs, &rhs) {
                    (x, Expr::Const(c)) if *c == 1.0 => x.clone(),
                    (Expr::Const(c), _) if *c == 1.0 => Expr::Const(1.0),
                    _ => Expr::Pow(Box::new(lhs), Box::new(rhs))
                }
            },
            _ => self.clone()
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
}
