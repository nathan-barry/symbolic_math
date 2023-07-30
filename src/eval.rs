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
