use types::*;
use std::convert;
use lisp::allocate::AllocObject;
use lisp;

pub enum IntoObject {
    Cons(ConsCell),
    Num(f64),
    Sym(Symbol),
    String(RlispString),
    Function(RlispFunc),
    Error(RlispError),
    Bool(bool),
}

pub trait ConvertIntoObject: AllocObject {
    fn convert_into_object(&mut self, i: IntoObject) -> Object {
        match i {
            IntoObject::Cons(c) => self.alloc(c),
            IntoObject::Num(n) => Object::from(n),
            IntoObject::Sym(s) => self.alloc(s),
            IntoObject::String(s) => self.alloc(s),
            IntoObject::Function(f) => self.alloc(f),
            IntoObject::Error(e) => self.alloc(e),
            IntoObject::Bool(b) => Object::from(b),
        }
    }
}

impl ConvertIntoObject for lisp::Lisp {}

impl convert::From<ConsCell> for IntoObject {
    fn from(c: ConsCell) -> Self {
        IntoObject::Cons(c)
    }
}

impl convert::From<f64> for IntoObject {
    fn from(n: f64) -> Self {
        IntoObject::Num(n)
    }
}

impl convert::From<Symbol> for IntoObject {
    fn from(s: Symbol) -> Self {
        IntoObject::Sym(s)
    }
}

impl convert::From<RlispString> for IntoObject {
    fn from(s: RlispString) -> Self {
        IntoObject::String(s)
    }
}

impl convert::From<RlispFunc> for IntoObject {
    fn from(f: RlispFunc) -> Self {
        IntoObject::Function(f)
    }
}

impl convert::From<RlispError> for IntoObject {
    fn from(e: RlispError) -> Self {
        IntoObject::Error(e)
    }
}

impl convert::From<bool> for IntoObject {
    fn from(b: bool) -> Self {
        IntoObject::Bool(b)
    }
}
