use std::collections::HashMap;
use std::default::Default;
use types::*;
use lisp::Lisp;

pub trait Symbols {
    fn intern<T>(&mut self, sym: T) -> &Symbol
    where
        ::std::string::String: ::std::convert::From<T>;
}

pub struct SymbolsTab {
    map: HashMap<String, Symbol>,
}

impl Default for SymbolsTab {
    fn default() -> Self {
        SymbolsTab {
            map: HashMap::new(),
        }
    }
}

impl Symbols for SymbolsTab {
    fn intern<T>(&mut self, sym: T) -> &Symbol
    where
        ::std::string::String: ::std::convert::From<T>,
    {
        let sym = String::from(sym);
        if !self.map.contains_key(&sym) {
            let new_symbol = Symbol::from_string(sym.clone());
            let _ = self.map.insert(sym.clone(), new_symbol);
        }
        if let Some(symbol) = self.map.get(&sym) {
            symbol
        } else {
            unreachable!()
        }
    }
}

impl Symbols for Lisp {
    fn intern<T>(&mut self, sym: T) -> &Symbol
    where
        ::std::string::String: ::std::convert::From<T>,
    {
        self.symbols.intern(sym)
    }
}
