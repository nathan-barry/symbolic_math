/// The 'Symbol' struct represents a symbolic variable
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol {
    pub name: String,
}

impl Symbol {
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
