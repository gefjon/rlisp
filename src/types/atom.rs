use std::fmt;
use std::convert;
use types::Object;

use super::Symbol;

#[derive(Clone)]
pub enum Atoms {
    Fixnum(isize),
    Float(f64),
    Sym(Symbol),
}

impl Atoms {
    pub fn symbol_from_str(sym: &str) -> Self {
        Atoms::Sym(Symbol::from_str(sym))
    }
}

impl fmt::Display for Atoms {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Atoms::Fixnum(n) => n.fmt(f),
            Atoms::Float(n) => n.fmt(f),
            Atoms::Sym(ref sym) => sym.fmt(f),
        }
    }
}

impl convert::From<Atoms> for Object {
    fn from(atom: Atoms) -> Self {
        Object::Atom(atom)
    }
}

impl convert::From<Symbol> for Atoms {
    fn from(sym: Symbol) -> Self {
        Atoms::Sym(sym)
    }
}

impl convert::From<f64> for Atoms {
    fn from(num: f64) -> Self {
        Atoms::Float(num)
    }
}

impl convert::From<isize> for Atoms {
    fn from(num: isize) -> Self {
        Atoms::Fixnum(num)
    }
}
