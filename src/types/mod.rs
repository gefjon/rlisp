use std::fmt;
use std::convert;
use std::boxed::Box;

pub mod string;
pub use self::string::RlispString;

pub mod symbol;
pub use self::symbol::Symbol;

pub mod cons;
pub use self::cons::ConsCell;

#[derive(Copy, Clone)]
pub enum Object {
    Cons(*const ConsCell),
    Num(f64),
    Sym(*const Symbol),
    String(*const RlispString),
    Nil,
}

impl Object {
    pub fn nil() -> Self {
        Object::Nil
    }
    pub fn symbolp(self) -> bool {
        if let Object::Sym(_) = self {
            true
        } else {
            false
        }
    }
    pub fn into_symbol<'unbound>(self) -> Option<&'unbound Symbol> {
        if let Object::Sym(ptr) = self {
            Some(unsafe { &(*ptr) })
        } else {
            None
        }
    }
    pub unsafe fn deallocate(self) {
        match self {
            Object::Num(_) | Object::Nil => (),
            Object::Cons(c) => {
                Box::from_raw(c as *mut ConsCell);
            }
            Object::Sym(s) => {
                Box::from_raw(s as *mut Symbol);
            }
            Object::String(s) => {
                Box::from_raw(s as *mut String);
            }
        }
    }
    pub fn gc_mark(self, marking: ::gc::GcMark) {
        match self {
            Object::Num(_) | Object::Nil => (),
            Object::Cons(c) => unsafe { (*(c as *mut ConsCell)).gc_mark(marking) },
            Object::Sym(s) => unsafe { (*(s as *mut Symbol)).gc_mark(marking) },
            Object::String(_s) => unimplemented!(),
        }
    }
    pub fn should_dealloc(self, current_marking: ::gc::GcMark) -> bool {
        match self {
            Object::Num(_) | Object::Nil => false,
            Object::Sym(s) => unsafe { (*s).should_dealloc(current_marking) },
            Object::Cons(c) => unsafe { (*c).should_dealloc(current_marking) },
            Object::String(_s) => unimplemented!(),
        }
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

impl convert::From<*const RlispString> for Object {
    fn from(string: *const RlispString) -> Self {
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
