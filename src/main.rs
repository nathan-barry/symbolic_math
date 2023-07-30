use std::collections::HashMap;
use symbolic_math::expr::Expr;
use symbolic_math::symbol::Symbol;

fn main() {
    let x = Expr::new_var("x");
    let y = Expr::new_var("y");
    let c = Expr::new_val(3.0);
    let p = Expr::new_val(2.0);
    let mut vars: HashMap<Symbol, f64> = HashMap::new();

    if let Expr::Symbol(s) = x.clone() {
        vars.insert(s, 1.0);
    }
    if let Expr::Symbol(s) = y.clone() {
        vars.insert(s, 2.0);
    }
    let res = (x+y*c.pow(p)).eval(&vars).unwrap();
    println!("{}", res);
}
