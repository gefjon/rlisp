use std::fmt;
use std::default::Default;
use std::convert;
use std::boxed::Box;
use gc::GarbageCollected;

pub mod string;
pub use self::string::RlispString;

pub mod symbol;
pub use self::symbol::Symbol;

pub mod cons;
pub use self::cons::ConsCell;

pub mod function;
pub use self::function::RlispFunc;

#[derive(Copy, Clone)]
pub enum Object {
    Cons(*const ConsCell),
    Num(f64),
    Sym(*const Symbol),
    String(*const RlispString),
    Function(*const RlispFunc),
    Bool(bool),
}

#[derive(Copy, Clone, Debug)]
pub enum RlispType {
    Cons,
    Num,
    Sym,
    String,
    Function,
    Bool,
}

impl Object {
    pub fn nil() -> Self {
        Object::Bool(false)
    }
    pub fn t() -> Self {
        Object::Bool(true)
    }
    pub fn boolp(self) -> bool {
        if let Object::Bool(_) = self {
            true
        } else {
            false
        }
    }
    pub fn symbolp(self) -> bool {
        if let Object::Sym(_) = self {
            true
        } else {
            false
        }
    }
    pub fn numberp(self) -> bool {
        if let Object::Num(_) = self {
            true
        } else {
            false
        }
    }
    pub fn consp(self) -> bool {
        if let Object::Cons(_) = self {
            true
        } else {
            false
        }
    }
    pub fn stringp(self) -> bool {
        if let Object::String(_) = self {
            true
        } else {
            false
        }
    }
    pub fn functionp(self) -> bool {
        if let Object::Function(_) = self {
            true
        } else {
            false
        }
    }
    pub fn nilp(self) -> bool {
        if let Object::Bool(false) = self {
            true
        } else {
            false
        }
    }
    pub fn what_type(self) -> RlispType {
        match self {
            Object::Cons(_) => RlispType::Cons,
            Object::Num(_) => RlispType::Num,
            Object::Sym(_) => RlispType::Sym,
            Object::String(_) => RlispType::String,
            Object::Function(_) => RlispType::Function,
            Object::Bool(_) => RlispType::Bool,
        }
    }
    pub fn into_symbol<'unbound>(self) -> Option<&'unbound Symbol> {
        if let Object::Sym(ptr) = self {
            Some(unsafe { &(*ptr) })
        } else {
            None
        }
    }
    pub fn into_symbol_mut<'unbound>(self) -> Option<&'unbound mut Symbol> {
        if let Object::Sym(ptr) = self {
            Some(unsafe { &mut (*(ptr as *mut Symbol)) })
        } else {
            None
        }
    }
    pub fn into_cons<'unbound>(self) -> Option<&'unbound ConsCell> {
        if let Object::Cons(ptr) = self {
            Some(unsafe { &(*ptr) })
        } else {
            None
        }
    }
    pub fn into_function<'unbound>(self) -> Option<&'unbound RlispFunc> {
        if let Object::Function(ptr) = self {
            Some(unsafe { &(*ptr) })
        } else {
            None
        }
    }
    pub unsafe fn deallocate(self) {
        match self {
            Object::Num(_) | Object::Bool(_) => (),
            Object::Cons(c) => {
                Box::from_raw(c as *mut ConsCell);
            }
            Object::Sym(s) => {
                Box::from_raw(s as *mut Symbol);
            }
            Object::String(s) => {
                Box::from_raw(s as *mut String);
            }
            Object::Function(f) => {
                Box::from_raw(f as *mut RlispFunc);
            }
        }
    }
    pub fn gc_mark(self, marking: ::gc::GcMark) {
        match self {
            Object::Num(_) | Object::Bool(_) => (),
            Object::Cons(c) => unsafe { (*(c as *mut ConsCell)).gc_mark(marking) },
            Object::Sym(s) => unsafe { (*(s as *mut Symbol)).gc_mark(marking) },
            Object::String(_s) => unimplemented!(),
            Object::Function(f) => unsafe { (*(f as *mut RlispFunc)).gc_mark(marking) },
        }
    }
    pub fn should_dealloc(self, current_marking: ::gc::GcMark) -> bool {
        match self {
            Object::Num(_) | Object::Bool(_) => false,
            Object::Sym(s) => unsafe { (*s).should_dealloc(current_marking) },
            Object::Cons(c) => unsafe { (*c).should_dealloc(current_marking) },
            Object::String(_s) => unimplemented!(),
            Object::Function(f) => unsafe { (*f).should_dealloc(current_marking) },
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Object::Bool(false) => write!(f, "nil"),
            Object::Bool(true) => write!(f, "t"),
            Object::Num(n) => write!(f, "{}", n),
            Object::Sym(s) => unsafe { write!(f, "{}", *s) },
            Object::Cons(c) => unsafe { write!(f, "{}", *c) },
            Object::String(s) => unsafe { write!(f, "\"{}\"", *s) },
            Object::Function(func) => unsafe { write!(f, "{}", *func) },
        }
    }
}

impl Default for Object {
    fn default() -> Self {
        Object::Bool(true)
    }
}

impl convert::From<Object> for bool {
    fn from(obj: Object) -> bool {
        !obj.nilp()
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

impl convert::From<*const RlispFunc> for Object {
    fn from(func: *const RlispFunc) -> Self {
        Object::Function(func)
    }
}

impl convert::From<*mut RlispString> for Object {
    fn from(string: *mut RlispString) -> Self {
        Object::String(string as _)
    }
}

impl convert::From<*mut ConsCell> for Object {
    fn from(cons: *mut ConsCell) -> Self {
        Object::Cons(cons as _)
    }
}

impl convert::From<*mut Symbol> for Object {
    fn from(sym: *mut Symbol) -> Self {
        Object::Sym(sym as _)
    }
}

impl convert::From<*mut RlispFunc> for Object {
    fn from(func: *mut RlispFunc) -> Self {
        Object::Function(func as _)
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
