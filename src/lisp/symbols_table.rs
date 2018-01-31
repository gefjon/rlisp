use types::*;
use types::rlisperror::RlispErrorKind;
use lisp::Lisp;
use lisp::allocate::AllocObject;

pub trait Symbols {
    fn intern<T>(&mut self, sym: T) -> Object
    where
        ::std::string::String: ::std::convert::From<T>,
        T: ::std::convert::AsRef<str>;
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
    fn error_name(&mut self, err: &RlispErrorKind) -> Object {
        self.intern(match *err {
            RlispErrorKind::WrongType { .. } => "wrong-type-error",
            RlispErrorKind::BadArgsCount { .. } => "wrong-arg-count-error",
            RlispErrorKind::ImproperList => "improper-list-error",
            RlispErrorKind::RustError(_) => "internal-error",
        })
    }
}

impl Symbols for Lisp {
    fn intern<T>(&mut self, sym: T) -> Object
    where
        ::std::string::String: ::std::convert::From<T>,
        T: ::std::convert::AsRef<str>,
    {
        let sym = String::from(sym);
        if sym == "nil" {
            Object::nil()
        } else if sym == "t" {
            Object::t()
        } else {
            if !self.symbols.contains_key(&sym) {
                let new_symbol = self.alloc_sym(sym.as_ref());
                let _ = self.symbols.insert(sym.clone(), new_symbol);
            }
            if let Some(symbol) = self.symbols.get(&sym) {
                *symbol
            } else {
                unreachable!()
            }
        }
    }
}
