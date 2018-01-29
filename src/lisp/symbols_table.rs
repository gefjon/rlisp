use types::*;
use lisp::Lisp;
use std::boxed::Box;

pub trait Symbols {
    fn intern<T>(&mut self, sym: T) -> Object
    where
        ::std::string::String: ::std::convert::From<T>;
    fn type_name(&mut self, typ: RlispType) -> Object {
        self.intern(match typ {
            RlispType::Cons => "cons",
            RlispType::Num => "number",
            RlispType::Sym => "symbol",
            RlispType::String => "string",
            RlispType::Function => "function",
            RlispType::Bool => "boolean",
            RlispType::Error => "error",
            RlispType::Integer => "integer",
            RlispType::NatNum => "natnum",
        })
    }
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
