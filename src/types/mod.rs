use std::rc::Rc;
use std::fmt;
use std::convert;

pub mod symbol;
pub use self::symbol::Symbol;

pub mod cons;
pub use self::cons::ConsCell;

#[derive(Clone)]
pub enum Object {
    Cons(Rc<ConsCell>),
    Num(f64),
    Sym(Symbol),
    String(Rc<String>),
    Nil,
}

impl Object {
    pub fn nil() -> Self {
        Object::Nil
    }
    pub fn cons(car: Object, cdr: Object) -> Self {
        Object::Cons(Rc::new(ConsCell::new(car, cdr)))
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Object::Nil => write!(f, "nil"),
            Object::Num(ref n) => write!(f, "{}", n),
            Object::Sym(ref s) => write!(f, "{}", s),
            Object::Cons(ref c) => write!(f, "{}", c),
            Object::String(ref s) => write!(f, "\"{}\"", s),
        }
    }
}

impl convert::From<String> for Object {
    fn from(string: String) -> Self {
        Object::String(Rc::new(string))
    }
}

impl convert::From<ConsCell> for Object {
    fn from(cons: ConsCell) -> Self {
        Object::Cons(Rc::new(cons))
    }
}

impl convert::From<Symbol> for Object {
    fn from(sym: Symbol) -> Self {
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
