use types::*;
use lisp::Lisp;
use std::boxed::Box;

pub trait Symbols {
    fn intern<T>(&mut self, sym: T) -> Object
    where
        ::std::string::String: ::std::convert::From<T>;
}

impl Symbols for Lisp {
    fn intern<T>(&mut self, sym: T) -> Object
    where
        ::std::string::String: ::std::convert::From<T>,
    {
        let sym = String::from(sym);
        if sym == "nil" {
            Object::nil()
        } else if sym == "t" {
            Object::t()
        } else {
            if !self.symbols.contains_key(&sym) {
                let new_symbol = Box::new(Symbol::from_string(sym.clone()));
                let _ = self.symbols.insert(sym.clone(), Box::into_raw(new_symbol));
            }
            if let Some(symbol) = self.symbols.get(&sym) {
                Object::Sym(*symbol)
            } else {
                unreachable!()
            }
        }
    }
}
