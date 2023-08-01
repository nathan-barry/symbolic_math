# Symbolic_Math

`symbolic_math` is a Rust library that facilitates representation and manipulation of mathematical expressions. The library offers structures to model various mathematical operations, including addition, subtraction, multiplication, division, and exponentiation. These expressions can contain constants, symbols, or other complex expressions. Furthermore, it provides tools to evaluate and simplify these expressions.

## Key Components

- `Expr`: An enum representing different types of mathematical expressions.
- `Symbol`: A struct representing a symbolic variable.

This library also provides several implementations for `Expr`, including:

- Constructors for creating new instances of `Expr`.
- A `Display` implementation to convert an `Expr` instance to a string.
- A `simplify` method to simplify an `Expr` instance.
- An `expand` method for basic expansion of an `Expr` instance.
- An `eval` method to evaluate an `Expr` instance.

The `operators` module includes operator overloads for `Expr`, enabling the combination of `Expr` instances using standard mathematical operators.

## Usage

To include `symbolic_math` in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
symbolic_math = "0.1.1"
```

You can then use it in your code as follows:

```rust
use symbolic_math::expr::Expr;
use symbolic_math::symbol::Symbol;
use std::collections::HashMap;

let x = Expr::new_var("x");
let y = Expr::new_var("y");
let z = Expr::new_var("z");
let res = (x.clone() + x.clone() + y.clone() * y.clone()).pow(z);
println!("{}", res);  // prints: "(2x + y^2)^z"
println!("{}", res.simplify());  // prints: "(2x + y^2)^z"

let mut vars: HashMap<Symbol, f64> = HashMap::new();
vars.insert(Symbol::new("x"), 4.0);
vars.insert(Symbol::new("y"), 3.0);
vars.insert(Symbol::new("z"), 2.0);
println!("{}", res.eval(&vars).unwrap()); // prints: "289"
```

For more detailed information on how to use `symbolic_math`, refer to the documentation for each individual type and method.
