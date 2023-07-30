use symbolic_math::expr::Expr;

fn main() {
    let x = Expr::new_var("x");
    let y = Expr::new_var("y");
    let c = Expr::new_val(72.0);
    let p = Expr::new_val(5.0);
    println!("{}", x);
    println!("{}", y);
    println!("{}", c);
    println!("{}", x+y*c.pow(p));
}
