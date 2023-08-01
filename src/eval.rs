use std::collections::HashMap;
use crate::expr::Expr;
use crate::symbol::Symbol;

#[derive(Debug)]
pub enum EvalError {
    SymbolNotFound(Symbol),
    UndefinedOperation,
}

impl Expr {
    pub fn eval(&self, vars: &HashMap<Symbol, f64>) -> Result<f64, EvalError> {
        match self {
            Expr::Const(c) => Ok(*c),
            Expr::Symbol(s) => vars.get(&s).cloned().ok_or(EvalError::SymbolNotFound(s.clone())),
            Expr::Add(lhs, rhs) => {
                let lhs_val = lhs.eval(vars)?;
                let rhs_val = rhs.eval(vars)?;
                Ok(lhs_val + rhs_val)
            }
            Expr::Sub(lhs, rhs) => {
                let lhs_val = lhs.eval(vars)?;
                let rhs_val = rhs.eval(vars)?;
                Ok(lhs_val - rhs_val)
            }
            Expr::Mul(lhs, rhs) => {
                let lhs_val = lhs.eval(vars)?;
                let rhs_val = rhs.eval(vars)?;
                Ok(lhs_val * rhs_val)
            }
            Expr::Div(lhs, rhs) => {
                let lhs_val = lhs.eval(vars)?;
                let rhs_val = rhs.eval(vars)?;
                Ok(lhs_val / rhs_val)
            }
            Expr::Pow(lhs, rhs) => {
                let base_val = lhs.eval(vars)?;
                let exp_val = rhs.eval(vars)?;
                let res = base_val.powf(exp_val);
                if res.is_nan() || res.is_infinite() {
                    Err(EvalError::UndefinedOperation)
                } else {
                    Ok(res)
                }
            }
            Expr::Neg(expr) => {
                let expr_val = expr.eval(vars)?;
                Ok(-expr_val)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn eval_basic_operations() {
        let x = Expr::new_var("x");
        let y = Expr::new_var("y");
        let mut vars: HashMap<Symbol, f64> = HashMap::new();
        vars.insert(x.get_symbol().unwrap(), 2.0);
        vars.insert(y.get_symbol().unwrap(), 3.0);

        let res_add = x.clone() + y.clone();
        assert_eq!(res_add.eval(&vars).unwrap(), 5.0);
        let res_sub = x.clone() - y.clone();
        assert_eq!(res_sub.eval(&vars).unwrap(), -1.0);
        let res_mul = x.clone() * y.clone();
        assert_eq!(res_mul.eval(&vars).unwrap(), 6.0);
        let res_div = y.clone() / x.clone();
        assert_eq!(res_div.eval(&vars).unwrap(), 1.5);

        let res_complicated = (res_add.pow(res_sub) * res_div) * res_mul;
        assert_eq!(res_complicated.eval(&vars).unwrap(), 1.8);
    }
}
