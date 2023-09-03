use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum SymbolScope {
    GLOBAL,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Symbol {
    pub name: String,
    pub scope: SymbolScope,
    pub index: u16,
}

impl Symbol {
    pub fn new(name: String, scope: SymbolScope, index: u16) -> Self {
        Self { name, scope, index }
    }
}

pub struct SymbolTable {
    store: HashMap<String, Symbol>,
    num_defs: u16,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
            num_defs: 0,
        }
    }

    pub fn define(&mut self, name: String) -> Symbol {
        let symbol = Symbol {
            name: name.clone(),
            scope: SymbolScope::GLOBAL,
            index: self.num_defs,
        };

        self.store.insert(name, symbol.clone());
        self.num_defs += 1;

        symbol
    }

    pub fn resolve(&self, name: String) -> Option<&Symbol> {
        self.store.get(&name)
    }
}

#[cfg(test)]
mod symbol_table_test {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_define() {
        let expected = HashMap::from([
            ("a", Symbol::new("a".to_string(), SymbolScope::GLOBAL, 0)),
            ("b", Symbol::new("b".to_string(), SymbolScope::GLOBAL, 1)),
        ]);

        let mut global = SymbolTable::new();

        let a = global.define("a".to_string());
        let b = global.define("b".to_string());
        assert_eq!(expected["a"], a);
        assert_eq!(expected["b"], b);
    }

    #[test]
    fn test_resolve_global() {
        let mut global = SymbolTable::new();
        global.define("a".to_string());
        global.define("b".to_string());

        let expected = vec![
            Symbol::new("a".to_string(), SymbolScope::GLOBAL, 0),
            Symbol::new("b".to_string(), SymbolScope::GLOBAL, 1),
        ];

        for symbol in expected {
            let s = global.resolve(symbol.name.to_string()).unwrap();
            assert_eq!(symbol, s.clone());
        }
    }
}
