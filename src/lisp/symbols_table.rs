use std::collections::HashMap;
use std::default::Default;
use types::*;
use lisp::Lisp;
use std::boxed::Box;

pub trait Symbols {
    fn intern<T>(&mut self, sym: T) -> Object
    where
        ::std::string::String: ::std::convert::From<T>;
}

pub struct SymbolsTab {
    pub map: HashMap<String, *const Symbol>,
}

impl Default for SymbolsTab {
    fn default() -> Self {
        SymbolsTab {
            map: HashMap::new(),
        }
    }
}

impl Symbols for SymbolsTab {
    fn intern<T>(&mut self, sym: T) -> Object
    where
        ::std::string::String: ::std::convert::From<T>,
    {
        let sym = String::from(sym);
        if sym == "nil" {
            Object::nil()
        } else {
            if !self.map.contains_key(&sym) {
                let new_symbol = Box::new(Symbol::from_string(sym.clone()));
                let _ = self.map.insert(sym.clone(), Box::into_raw(new_symbol));
            }
            if let Some(symbol) = self.map.get(&sym) {
                Object::Sym(*symbol)
            } else {
                unreachable!()
            }
        }
    }
}

impl Symbols for Lisp {
    fn intern<T>(&mut self, sym: T) -> Object
    where
        ::std::string::String: ::std::convert::From<T>,
    {
        self.symbols.intern(sym)
    }
}
