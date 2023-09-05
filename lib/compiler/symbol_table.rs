use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, PartialEq, Clone)]
pub enum SymbolScope {
    GLOBAL,
    LOCAL,
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

#[derive(Debug, Clone, Default)]
pub struct SymbolTable {
    pub outer: Option<Rc<RefCell<SymbolTable>>>,
    store: HashMap<String, Symbol>,
    pub num_defs: u16,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
            num_defs: 0,
            outer: None,
        }
    }

    pub fn new_enclosed(table: Rc<RefCell<SymbolTable>>) -> Self {
        Self {
            store: HashMap::new(),
            num_defs: 0,
            outer: Some(table),
        }
    }

    pub fn define(&mut self, name: String) -> Symbol {
        let scope = match self.outer {
            Some(_) => SymbolScope::LOCAL,
            None => SymbolScope::GLOBAL,
        };

        let symbol = Symbol {
            name: name.clone(),
            scope,
            index: self.num_defs,
        };

        self.store.insert(name, symbol.clone());
        self.num_defs += 1;

        symbol
    }

    pub fn resolve(&self, name: String) -> Option<Symbol> {
        match self.store.get(&name) {
            Some(o) => Some(o.clone()),
            None => match self.outer {
                Some(ref parent_env) => {
                    let env = parent_env.borrow();
                    env.resolve(name)
                }
                None => None,
            },
        }
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
            ("c", Symbol::new("c".to_string(), SymbolScope::LOCAL, 0)),
            ("d", Symbol::new("d".to_string(), SymbolScope::LOCAL, 1)),
            ("e", Symbol::new("e".to_string(), SymbolScope::LOCAL, 0)),
            ("f", Symbol::new("f".to_string(), SymbolScope::LOCAL, 1)),
        ]);

        let mut global = SymbolTable::new();

        let a = global.define("a".to_string());
        let b = global.define("b".to_string());
        assert_eq!(expected["a"], a);
        assert_eq!(expected["b"], b);

        let mut first_local = SymbolTable::new_enclosed(Rc::new(RefCell::new(global)));

        let c = first_local.define("c".to_string());
        let d = first_local.define("d".to_string());
        assert_eq!(expected["c"], c);
        assert_eq!(expected["d"], d);

        let mut second_local = SymbolTable::new_enclosed(Rc::new(RefCell::new(first_local)));

        let e = second_local.define("e".to_string());
        let f = second_local.define("f".to_string());
        assert_eq!(expected["e"], e);
        assert_eq!(expected["f"], f);
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

    #[test]
    fn test_resolve_local() {
        let mut global = SymbolTable::new();
        global.define("a".to_string());
        global.define("b".to_string());

        let mut local = SymbolTable::new_enclosed(Rc::new(RefCell::new(global)));
        local.define("c".to_string());
        local.define("d".to_string());

        let expected = vec![
            Symbol::new("a".to_string(), SymbolScope::GLOBAL, 0),
            Symbol::new("b".to_string(), SymbolScope::GLOBAL, 1),
            Symbol::new("c".to_string(), SymbolScope::LOCAL, 0),
            Symbol::new("d".to_string(), SymbolScope::LOCAL, 1),
        ];

        for symbol in expected {
            let s = local.resolve(symbol.name.to_string()).unwrap();
            assert_eq!(symbol, s.clone());
        }
    }

    #[test]
    fn test_resolve_nested_local() {
        let mut global = SymbolTable::new();
        global.define("a".to_string());
        global.define("b".to_string());

        let mut first_local = SymbolTable::new_enclosed(Rc::new(RefCell::new(global)));
        first_local.define("c".to_string());
        first_local.define("d".to_string());

        let mut second_local =
            SymbolTable::new_enclosed(Rc::new(RefCell::new(first_local.clone())));
        second_local.define("e".to_string());
        second_local.define("f".to_string());

        let first_expected = vec![
            Symbol::new("a".to_string(), SymbolScope::GLOBAL, 0),
            Symbol::new("b".to_string(), SymbolScope::GLOBAL, 1),
            Symbol::new("c".to_string(), SymbolScope::LOCAL, 0),
            Symbol::new("d".to_string(), SymbolScope::LOCAL, 1),
        ];

        let second_expected = vec![
            Symbol::new("a".to_string(), SymbolScope::GLOBAL, 0),
            Symbol::new("b".to_string(), SymbolScope::GLOBAL, 1),
            Symbol::new("e".to_string(), SymbolScope::LOCAL, 0),
            Symbol::new("f".to_string(), SymbolScope::LOCAL, 1),
        ];

        for symbol in first_expected {
            let s = first_local.resolve(symbol.name.to_string()).unwrap();
            assert_eq!(symbol, s.clone());
        }

        for symbol in second_expected {
            let s = second_local.resolve(symbol.name.to_string()).unwrap();
            assert_eq!(symbol, s.clone());
        }
    }
}