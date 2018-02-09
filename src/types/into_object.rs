use types::*;
use std::convert;
use lisp::allocate::AllocObject;
use lisp;

pub enum IntoObject {
    Cons(ConsCell),
    Num(f64),
    String(&'static str),
    Function(RlispFunc),
    Error(RlispError),
    Bool(bool),
}

pub trait ConvertIntoObject: AllocObject {
    fn convert_into_object(&mut self, i: IntoObject) -> Object {
        match i {
            IntoObject::Cons(c) => self.alloc(c),
            IntoObject::Num(n) => Object::from(n),
            IntoObject::String(s) => self.alloc_string(s),
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

impl convert::From<&'static str> for IntoObject {
    fn from(s: &'static str) -> Self {
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

impl convert::From<i32> for IntoObject {
    fn from(n: i32) -> Self {
        IntoObject::Num(f64::from(n))
    }
}

impl convert::From<u32> for IntoObject {
    fn from(n: u32) -> Self {
        IntoObject::Num(f64::from(n))
    }
}
