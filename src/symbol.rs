/// Represents a symbolic variable in a mathematical expression.
///
/// `Symbol` holds a `String` that is its name. It provides functionality to
/// create a new `Symbol` from a string slice. 
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol {
    /// The name of the symbolic variable.
    pub name: String,
}

impl Symbol {
    /// Creates a new `Symbol` from a string slice.
    ///
    /// The function takes a string slice as input and returns a `Symbol` with the input as its name.
    ///
    /// Note: Should normally use s.get_symbol() for Expr::Symbol(s) instead.
    pub fn new(name: &str) -> Symbol {
        Symbol { name: name.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_name() {
        let symbol = Symbol::new("x");
        assert_eq!(symbol.name, "x");
    }
}
