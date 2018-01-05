use std::rc::Rc;
use std::fmt;
use std::convert;

pub mod symbol;
pub use self::symbol::Symbol;

pub mod atom;
pub use self::atom::Atoms;

pub mod cons;
pub use self::cons::ConsCell;

#[derive(Clone)]
pub enum Collections {
    Cons(Rc<ConsCell>),
}

impl fmt::Display for Collections {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Collections::Cons(ref c) => write!(f, "{}", *c),
        }
    }
}

#[derive(Clone)]
pub enum Object {
    Atom(Atoms),
    Collection(Collections),
    Nil,
}

impl Object {
    pub fn nil() -> Self {
        Object::Nil
    }
    pub fn cons(car: Object, cdr: Object) -> Self {
        ConsCell::new(car, cdr)
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Object::Nil => write!(f, "nil"),
            Object::Atom(ref a) => write!(f, "{}", a),
            Object::Collection(ref c) => write!(f, "{}", c),
        }
    }
}

impl convert::From<Symbol> for Object {
    fn from(sym: Symbol) -> Self {
        Object::from(Atoms::from(sym))
    }
}

impl convert::From<f64> for Object {
    fn from(num: f64) -> Self {
        Object::from(Atoms::from(num))
    }
}

impl convert::From<isize> for Object {
    fn from(num: isize) -> Self {
        Object::from(Atoms::from(num))
    }
}
