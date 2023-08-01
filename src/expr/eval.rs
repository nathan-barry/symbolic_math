use std::collections::HashMap;
use crate::expr::Expr;
use crate::symbol::Symbol;

/// Enum representing possible errors that can occur while evaluating an expression.
#[derive(Debug)]
pub enum EvalError {
    SymbolNotFound(Symbol),
    UndefinedOperation,
}

impl Expr {
    /// Evaluates the current expression using the given map of symbols to values.
    ///
    /// If an error occurs during the evaluation, such as not finding a symbol in the map
    /// or attempting an undefined operation, it returns an `Err(EvalError)`.
    ///
    /// # Arguments
    ///
    /// * `&self` - A reference to the current instance of `Expr`.
    /// * `vars` - A map from symbols to their corresponding values.
    ///
    /// # Example
    ///
    /// ```
    /// use symbolic_math::expr::Expr;
    /// use symbolic_math::symbol::Symbol;
    /// use std::collections::HashMap;
    ///
    /// let x = Expr::new_var("x");
    /// let y = Expr::new_var("y");
    /// let expr = x * y;
    /// let mut vars = HashMap::new();
    /// vars.insert(Symbol::new("x"), 3.0);
    /// vars.insert(Symbol::new("y"), 9.0);
    /// assert_eq!(expr.eval(&vars).unwrap(), 27.0);
    /// ```
    pub fn eval(&self, vars: &HashMap<Symbol, f64>) -> Result<f64, EvalError> {
        match self {
            Expr::Const(c) => Ok(*c),
            Expr::Symbol(s) => vars.get(&s).cloned().ok_or(EvalError::SymbolNotFound(s.clone())),
            Expr::Add(lhs, rhs) => {
                let lhs_val = lhs.eval(vars)?;
                let rhs_val = rhs.eval(vars)?;
                Ok(round(lhs_val + rhs_val))
            }
            Expr::Sub(lhs, rhs) => {
                let lhs_val = lhs.eval(vars)?;
                let rhs_val = rhs.eval(vars)?;
                Ok(round(lhs_val - rhs_val))
            }
            Expr::Mul(lhs, rhs) => {
                let lhs_val = lhs.eval(vars)?;
                let rhs_val = rhs.eval(vars)?;
                Ok(round(lhs_val * rhs_val))
            }
            Expr::Div(lhs, rhs) => {
                let lhs_val = lhs.eval(vars)?;
                let rhs_val = rhs.eval(vars)?;
                Ok(round(lhs_val / rhs_val))
            }
            Expr::Pow(lhs, rhs) => {
                let base_val = lhs.eval(vars)?;
                let exp_val = rhs.eval(vars)?;
                let res = base_val.powf(exp_val);
                if res.is_nan() || res.is_infinite() {
                    Err(EvalError::UndefinedOperation)
                } else {
                    Ok(round(res))
                }
            }
            Expr::Neg(expr) => {
                let expr_val = expr.eval(vars)?;
                Ok(-expr_val)
            }
        }
    }
}

/// Rounds a given `f64` value to the 14th decimal place.
///
/// This function is used in the `eval` method above to round the results of floating
/// point operations, mitigating the effects of floating point precision errors.
///
/// # Arguments
///
/// * `val` - The `f64` value to be rounded.
fn round(val: f64) -> f64 {
    (val * 10e14).round() / 10e14
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
