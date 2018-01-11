use std::fmt;
use std::convert;

pub mod symbol;
pub use self::symbol::Symbol;

pub mod cons;
pub use self::cons::ConsCell;

#[derive(Copy, Clone)]
pub enum Object {
    Cons(*const ConsCell),
    Num(f64),
    Sym(*const Symbol),
    String(*const String),
    Nil,
}

impl Object {
    pub fn nil() -> Self {
        Object::Nil
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Object::Nil => write!(f, "nil"),
            Object::Num(n) => write!(f, "{}", n),
            Object::Sym(s) => unsafe { write!(f, "{}", *s) },
            Object::Cons(c) => unsafe { write!(f, "{}", *c) },
            Object::String(s) => unsafe { write!(f, "\"{}\"", *s) },
        }
    }
}

impl convert::From<*const String> for Object {
    fn from(string: *const String) -> Self {
        Object::String(string)
    }
}

impl convert::From<*const ConsCell> for Object {
    fn from(cons: *const ConsCell) -> Self {
        Object::Cons(cons)
    }
}

impl convert::From<*const Symbol> for Object {
    fn from(sym: *const Symbol) -> Self {
        Object::Sym(sym)
    }
}

impl convert::From<f64> for Object {
    fn from(num: f64) -> Self {
        Object::Num(num)
    }
}

impl convert::From<isize> for Object {
    fn from(num: isize) -> Self {
        Object::from(num as f64)
    }
}
