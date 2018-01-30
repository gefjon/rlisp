/*
Rlisp's Object types are Copy and are passed by value. bools and
Numbers (f64s) are passed by value, but everything else is included as
a *const T in Object. Many functions mutate the things pointed to
after casting to *mut T, but the pointers are stored as *const T to
encourage cloning over mutation.

It doesn't make a lot of sense to store Symbols as *const Symbol,
since they are frequently mutated (rebound), or functions, since they
implement FnMut, and those types could in the future could be changed
to *mut T, but for now they are *const for consistence.
*/
use result::*;
use std::{cmp, convert, fmt};
use std::default::Default;
use std::boxed::Box;
use gc::GarbageCollected;

pub mod string;
pub use self::string::RlispString;

pub mod symbol;
pub use self::symbol::Symbol;

pub mod cons;
pub use self::cons::ConsCell;

pub mod rlisperror;
pub use self::rlisperror::RlispError;

pub mod function;
pub use self::function::RlispFunc;

pub mod conversions;
use self::conversions::*;

pub mod into_object;

#[derive(Copy, Clone)]
pub enum Object {
    Cons(*const ConsCell),
    Num(f64),
    Sym(*const Symbol),
    String(*const RlispString),
    Function(*const RlispFunc),
    Error(*const RlispError),
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
    Error,
    Integer,
    NatNum,
}

impl Object {
    pub fn nil() -> Self {
        // returns the object which the symbol `nil` evauluates to
        Object::Bool(false)
    }
    pub fn t() -> Self {
        // returns the object which the symbol `t` evaluates to
        Object::Bool(true)
    }
    pub fn boolp(self) -> bool {
        // true if self is a bool. note that any object can be cast to
        // bool, and every object other than `nil` evaluates to true,
        // but that this method treats only exactly `t` and `nil` as
        // bools, and returns false for any other Object.
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
        // note that being a cons does not mean being a proper
        // list. listp is a more expensive (and as yet unimplemented)
        // operation which involves traversing the list to check that
        // it is nil-terminated.
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
        // the logical inverse of casting an Object to bool; true iff
        // self == Object::nil().
        if let Object::Bool(false) = self {
            true
        } else {
            false
        }
    }
    pub fn what_type(self) -> RlispType {
        // this is basically a cleaner version of mem::discriminant
        // for Objects
        match self {
            Object::Cons(_) => RlispType::Cons,
            Object::Num(_) => RlispType::Num,
            Object::Sym(_) => RlispType::Sym,
            Object::String(_) => RlispType::String,
            Object::Function(_) => RlispType::Function,
            Object::Bool(_) => RlispType::Bool,
            Object::Error(_) => RlispType::Error,
        }
    }
    pub unsafe fn deallocate(self) {
        // this should only ever be called by the garbage collector!
        // potential future change: move from being a pub Object
        // method to being a private function in gc

        debug!("deallocating {:?}", self);
        match self {
            Object::Num(_) | Object::Bool(_) => debug!("deallocating {} is a no-op", self),
            Object::Cons(c) => {
                debug!("building a box from {:?}", c);
                Box::from_raw(c as *mut ConsCell);
                debug!("box built, will drop it");
            }
            Object::Sym(s) => {
                debug!("building a box from {:?}", s);
                Box::from_raw(s as *mut Symbol);
                debug!("box built, will drop it");
            }
            Object::String(s) => {
                debug!("building a box from {:?}", s);
                Box::from_raw(s as *mut RlispString);
                debug!("box built, will drop it");
            }
            Object::Function(f) => {
                debug!("building a box from {:?}", f);
                Box::from_raw(f as *mut RlispFunc);
                debug!("box built, will drop it");
            }
            Object::Error(e) => {
                Box::from_raw(e as *mut RlispFunc);
            }
        }
    }
    pub fn gc_mark(self, marking: ::gc::GcMark) {
        // Object could probably implement gc::GarbageCollected, but
        // as of now it doesn't. Because of that, it instead has this
        // method and should_dealloc which mimic
        // GarbageCollected::{gc_mark, should_dealloc} and are called
        // by various types' gc_mark_children methods.
        match self {
            Object::Num(_) | Object::Bool(_) => (),
            Object::Cons(c) => unsafe { (*(c as *mut ConsCell)).gc_mark(marking) },
            Object::Sym(s) => unsafe { (*(s as *mut Symbol)).gc_mark(marking) },
            Object::String(s) => unsafe { (*(s as *mut RlispString)).gc_mark(marking) },
            Object::Function(f) => unsafe { (*(f as *mut RlispFunc)).gc_mark(marking) },
            Object::Error(e) => unsafe { (*(e as *mut RlispError)).gc_mark(marking) },
        }
    }
    pub fn should_dealloc(self, current_marking: ::gc::GcMark) -> bool {
        match self {
            Object::Num(_) | Object::Bool(_) => false,
            Object::Sym(s) => unsafe { (*s).should_dealloc(current_marking) },
            Object::Cons(c) => unsafe { (*c).should_dealloc(current_marking) },
            Object::String(s) => unsafe { (*s).should_dealloc(current_marking) },
            Object::Function(f) => unsafe { (*f).should_dealloc(current_marking) },
            Object::Error(e) => unsafe { (*e).should_dealloc(current_marking) },
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
            Object::String(s) => unsafe { write!(f, "{}", *s) },
            Object::Function(func) => unsafe { write!(f, "{}", *func) },
            Object::Error(e) => unsafe { write!(f, "{}", *e) },
        }
    }
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Object::Bool(false) => write!(f, "nil"),
            Object::Bool(true) => write!(f, "t"),
            Object::Num(n) => write!(f, "{}", n),
            Object::Sym(s) => unsafe { write!(f, "{}", *s) },
            Object::Cons(c) => unsafe { write!(f, "{:?}", *c) },
            Object::String(s) => unsafe { write!(f, "{:?}", *s) },
            Object::Function(func) => unsafe { write!(f, "{:?}", *func) },
            Object::Error(e) => unsafe { write!(f, "{}", *e) },
        }
    }
}

impl Default for Object {
    // the default Object is `t`
    fn default() -> Self {
        Object::Bool(true)
    }
}

impl convert::From<Object> for bool {
    // in a lisp, every Object except `nil` evaluates true
    fn from(obj: Object) -> bool {
        !(obj.nilp() && <&RlispError as FromObject>::is_type(obj))
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

impl convert::From<*const RlispError> for Object {
    fn from(err: *const RlispError) -> Self {
        Object::Error(err)
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

impl convert::From<*mut RlispError> for Object {
    fn from(err: *mut RlispError) -> Self {
        Object::Error(err as _)
    }
}

impl convert::From<bool> for Object {
    fn from(b: bool) -> Self {
        Object::Bool(b)
    }
}

impl convert::From<f64> for Object {
    fn from(num: f64) -> Self {
        Object::Num(num)
    }
}

impl convert::From<usize> for Object {
    // used for funcalls and such
    fn from(num: usize) -> Self {
        Object::from(num as f64)
    }
}

impl convert::From<isize> for Object {
    // this trait is kind of meaningless since all numbers in Rlisp
    // are currently f64s, but if the language ever gets an int type
    // for optimization, this may be useful.
    fn from(num: isize) -> Self {
        Object::from(num as f64)
    }
}

impl cmp::PartialEq for Object {
    fn eq(&self, other: &Object) -> bool {
        if let (&Object::Cons(lhs), &Object::Cons(rhs)) = (self, other) {
            lhs == rhs
        } else if let (&Object::Num(lhs), &Object::Num(rhs)) = (self, other) {
            lhs == rhs
        } else if let (&Object::Sym(lhs), &Object::Sym(rhs)) = (self, other) {
            lhs == rhs
        } else if let (&Object::String(lhs), &Object::String(rhs)) = (self, other) {
            lhs == rhs
        } else if let (&Object::Function(lhs), &Object::Function(rhs)) = (self, other) {
            lhs == rhs
        } else if let (&Object::Bool(lhs), &Object::Bool(rhs)) = (self, other) {
            lhs == rhs
        } else {
            false
        }
    }
}
