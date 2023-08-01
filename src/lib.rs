//! # Expr Library
//!
//! `expr_lib` is a library for representing and manipulating mathematical expressions. It provides structures
//! to model different mathematical operations including addition, subtraction, multiplication, division, and power. 
//! The expressions can be comprised of constants, symbols, or other expressions. The library also provides facilities 
//! for evaluating and simplifying these expressions.
//!
//! The main types provided by this library are:
//! 
//! * `Expr`: An enum representing different kinds of mathematical expressions.
//! * `Symbol`: A struct representing a symbolic variable.
//! * `EvalError`: An enum representing the types of errors that can occur when evaluating an expression.
//! 
//! The library also provides several implementations for `Expr`:
//!
//! * Constructors for creating new `Expr` instances.
//! * A `Display` implementation for converting an `Expr` to a string.
//! * A `simplify` method for simplifying an `Expr`.
//! * An `eval` method for evaluating an `Expr`.
//!
//! The library also includes operator overloads for `Expr`, located in the `operators` module, 
//! which allow `Expr` instances to be combined using standard mathematical operators.
//!
//! ## Examples
//! 
//! To use this library, add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! symbolic_math = "0.1.0"
//! ```
//!
//! Then, you can use it in your code like so:
//!
//! ```rust
//! use symbolic_math::expr::Expr;
//!
//! let x = Expr::new_var("x");
//! let y = Expr::new_var("x");
//! let z = Expr::new_var("x");
//! let res = (x.clone() + x.clone() + y.clone() * y.clone()).pow(z);
//! println!("{}", res);  // prints: "(2x + y^2)^z"
//! ```
//!
//! See the documentation for each individual type and method for more information on how to use this library.

pub mod symbol;
pub mod expr;

