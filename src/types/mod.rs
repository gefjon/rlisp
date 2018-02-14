/*
The Object type is a NaNbox. f64s are stored by value, and everything
else is stored in the unused 52 bits of NaN values. The high 4 bits of
that 52 are used as a tag (the enum ObjectTag defines the tag values),
and the low 48 store either a pointer or bool or integer immediate
value. Future improvements:

- use the low 3 bits of pointers as an additional tag (all x86
  pointers are 8-byte aligned)

- do something with the wasted bits in int and bool immediates (Rlisp
  ints are 32 bits, so there's an extra 16 in there)

These two changes could drastically increase the number of first-class
types Object can store, which would be nice.

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

pub mod namespace;
pub use self::namespace::{Namespace, Scope};

pub mod conversions;
use self::conversions::*;

pub mod into_object;

const NAN_MASK: u64 = 0b111_1111_1111 << 52; // Any NaN has these bits set
const _MAX_PTR: u64 = 1 << 48; // x86_64 pointers always fit in 48 bits; this is used in a debug_assert
const OBJECT_TAG_MASK: u64 = 0b1111 << 48; // for type-checking Objects

#[derive(Copy, Clone)]
pub struct Object(u64);

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum ObjectTag {
    // it is important that the first tag (0b0000 << 48) is a pointer
    // type because a tag of 0b0000 and a value of 0 or 1 denotes
    // numeric Infinity or NaN. Because the pointers 0x0 and 0x1 are
    // not valid, this is not an issue; if the tag 0b0000 denoted the
    // type Integer, we would have to choose between not being able to
    // represent Infinity and NaN or not being able to represent 0 and
    // 1, which would be a problem. As is, if allocating a Cons fails
    // and returns a nullptr, we get the float Infinity. Easy fix:
    // `panic` on failure to alloc
    Cons,
    Sym,
    String,
    Function,
    Error,
    Namespace,
    Integer,
    Bool,
}

impl convert::From<ObjectTag> for u64 {
    fn from(t: ObjectTag) -> u64 {
        ((t as u64) << 48)
    }
}

impl ObjectTag {
    fn tag(self, ptr: u64) -> u64 {
        debug_assert!(ptr < _MAX_PTR);
        let tagged = u64::from(self) ^ NAN_MASK ^ ptr;
        debug!("tagged the {:?}*\n{:#066b} as\n{:#066b}", self, ptr, tagged);
        tagged
    }
    fn is_of_type(self, ptr: u64) -> bool {
        let res = !Object::numberp(Object(ptr)) && (ptr & OBJECT_TAG_MASK) == u64::from(self);
        if res {
            debug!("{:#066b} is of type {:?}*", ptr, self);
        } else {
            debug!("{:#066b} is not of type {:?}*", ptr, self);
        }
        debug!(
            "{:#066b} is the mask associated with type {:?}*",
            u64::from(self),
            self
        );
        res
    }
    fn untag(self, ptr: u64) -> u64 {
        debug_assert!(self.is_of_type(ptr));
        let untagged = ptr & !(u64::from(self) ^ NAN_MASK);
        debug!(
            "untagged the {:?}*\n{:#066b} as \n{:#066b}",
            self, ptr, untagged
        );
        untagged
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RlispType {
    Cons,
    Num,
    Sym,
    String,
    Function,
    Bool,
    Error,
    Integer,
    Namespace,
}

impl Object {
    fn the_nan() -> u64 {
        f64::to_bits(::std::f64::NAN)
    }
    fn is_nanbox(self) -> bool {
        f64::from_bits(self.0).is_nan() && !self.nanp()
    }
    fn nanp(self) -> bool {
        self.0 == Self::the_nan()
    }
    pub fn nil() -> Self {
        // returns the object which the symbol `nil` evauluates to
        Object(ObjectTag::Bool.tag(0))
    }
    pub fn t() -> Self {
        // returns the object which the symbol `t` evaluates to
        Object(ObjectTag::Bool.tag(1))
    }
    pub fn boolp(self) -> bool {
        // true if self is a bool. note that any object can be cast to
        // bool, and every object other than `nil` evaluates to true,
        // but that this method treats only exactly `t` and `nil` as
        // bools, and returns false for any other Object.
        ObjectTag::Bool.is_of_type(self.0)
    }
    pub fn integerp(self) -> bool {
        ObjectTag::Integer.is_of_type(self.0)
    }
    pub fn symbolp(self) -> bool {
        ObjectTag::Sym.is_of_type(self.0)
    }
    pub fn numberp(self) -> bool {
        !self.is_nanbox()
    }
    pub fn consp(self) -> bool {
        // note that being a cons does not mean being a proper
        // list. listp is a more expensive (and as yet unimplemented)
        // operation which involves traversing the list to check that
        // it is nil-terminated.
        ObjectTag::Cons.is_of_type(self.0)
    }
    pub fn stringp(self) -> bool {
        ObjectTag::String.is_of_type(self.0)
    }
    pub fn functionp(self) -> bool {
        ObjectTag::Function.is_of_type(self.0)
    }
    pub fn errorp(self) -> bool {
        ObjectTag::Error.is_of_type(self.0)
    }
    pub fn namespacep(self) -> bool {
        ObjectTag::Namespace.is_of_type(self.0)
    }
    pub fn nilp(self) -> bool {
        // the logical inverse of casting an Object to bool; true iff
        // self == Object::nil().
        if let Some(b) = bool::maybe_from(self) {
            !b
        } else {
            false
        }
    }
    pub fn what_type(self) -> RlispType {
        if self.numberp() {
            RlispType::Num
        } else if self.integerp() {
            RlispType::Integer
        } else if self.consp() {
            RlispType::Cons
        } else if self.symbolp() {
            RlispType::Sym
        } else if self.stringp() {
            RlispType::String
        } else if self.functionp() {
            RlispType::Function
        } else if self.boolp() {
            RlispType::Bool
        } else if self.errorp() {
            RlispType::Error
        } else if self.namespacep() {
            RlispType::Namespace
        } else {
            unreachable!()
        }
    }
    pub fn gc_mark(self, marking: ::gc::GcMark) {
        // Object could probably implement gc::GarbageCollected, but
        // as of now it doesn't. Because of that, it instead has this
        // method and should_dealloc which mimic
        // GarbageCollected::{gc_mark, should_dealloc} and are called
        // by various types' gc_mark_children methods.
        unsafe {
            match self.what_type() {
                RlispType::Num | RlispType::Integer | RlispType::Bool => (),
                RlispType::Cons => {
                    <&mut ConsCell>::from_unchecked(self).gc_mark(marking);
                }
                RlispType::Sym => {
                    <&mut Symbol>::from_unchecked(self).gc_mark(marking);
                }
                RlispType::String => {
                    <&mut RlispString>::from_unchecked(self).gc_mark(marking);
                }
                RlispType::Function => {
                    <&mut RlispFunc>::from_unchecked(self).gc_mark(marking);
                }
                RlispType::Error => {
                    <&mut RlispError>::from_unchecked(self).gc_mark(marking);
                }
                RlispType::Namespace => {
                    <&mut Namespace>::from_unchecked(self).gc_mark(marking);
                }
            }
        }
    }
    pub fn should_dealloc(self, marking: ::gc::GcMark) -> bool {
        unsafe {
            match self.what_type() {
                RlispType::Num | RlispType::Integer | RlispType::Bool => false,
                RlispType::Cons => <&mut ConsCell>::from_unchecked(self).should_dealloc(marking),
                RlispType::Sym => <&mut Symbol>::from_unchecked(self).should_dealloc(marking),
                RlispType::String => {
                    <&mut RlispString>::from_unchecked(self).should_dealloc(marking)
                }
                RlispType::Function => {
                    <&mut RlispFunc>::from_unchecked(self).should_dealloc(marking)
                }
                RlispType::Error => <&mut RlispError>::from_unchecked(self).should_dealloc(marking),
                RlispType::Namespace => {
                    <&mut Namespace>::from_unchecked(self).should_dealloc(marking)
                }
            }
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            match self.what_type() {
                RlispType::Num => write!(f, "{}", f64::from_unchecked(*self)),
                RlispType::Integer => write!(f, "{}", i32::from_unchecked(*self)),
                RlispType::Bool => {
                    if self.nilp() {
                        write!(f, "nil")
                    } else {
                        write!(f, "t")
                    }
                }
                RlispType::Cons => write!(f, "{}", <&ConsCell>::from_unchecked(*self)),
                RlispType::Sym => write!(f, "{}", <&Symbol>::from_unchecked(*self)),
                RlispType::String => write!(f, "{}", <&RlispString>::from_unchecked(*self)),
                RlispType::Function => write!(f, "{}", <&RlispFunc>::from_unchecked(*self)),
                RlispType::Error => write!(f, "{}", <&RlispError>::from_unchecked(*self)),
                RlispType::Namespace => write!(f, "{}", <&Namespace>::from_unchecked(*self)),
            }
        }
    }
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            match self.what_type() {
                RlispType::Num => write!(f, "{}", f64::from_unchecked(*self)),
                RlispType::Integer => write!(f, "{}", i32::from_unchecked(*self)),
                RlispType::Bool => {
                    if self.nilp() {
                        write!(f, "nil")
                    } else {
                        write!(f, "t")
                    }
                }
                RlispType::Cons => write!(f, "{:?}", <&ConsCell>::from_unchecked(*self)),
                RlispType::Sym => write!(f, "{}", <&Symbol>::from_unchecked(*self)),
                RlispType::String => write!(f, "{:?}", <&RlispString>::from_unchecked(*self)),
                RlispType::Function => write!(f, "{:?}", <&RlispFunc>::from_unchecked(*self)),
                RlispType::Error => write!(f, "{}", <&RlispError>::from_unchecked(*self)),
                RlispType::Namespace => write!(f, "{:?}", <&Namespace>::from_unchecked(*self)),
            }
        }
    }
}

impl Default for Object {
    // the default Object is `t`
    fn default() -> Self {
        Object::t()
    }
}

impl convert::From<Object> for bool {
    // all values except `nil` and errors evaluate to true
    fn from(obj: Object) -> bool {
        !(obj.nilp() && ObjectTag::Error.is_of_type(obj.0))
    }
}

impl convert::From<*const RlispString> for Object {
    fn from(ptr: *const RlispString) -> Self {
        let ptr = ptr as u64;
        Object(ObjectTag::String.tag(ptr))
    }
}

impl convert::From<*const ConsCell> for Object {
    fn from(ptr: *const ConsCell) -> Self {
        let ptr = ptr as u64;
        Object(ObjectTag::Cons.tag(ptr))
    }
}

impl convert::From<*const Symbol> for Object {
    fn from(ptr: *const Symbol) -> Self {
        let ptr = ptr as u64;
        Object(ObjectTag::Sym.tag(ptr))
    }
}

impl convert::From<*const RlispFunc> for Object {
    fn from(ptr: *const RlispFunc) -> Self {
        let ptr = ptr as u64;
        Object(ObjectTag::Function.tag(ptr))
    }
}

impl convert::From<*const RlispError> for Object {
    fn from(ptr: *const RlispError) -> Self {
        let ptr = ptr as u64;
        Object(ObjectTag::Error.tag(ptr))
    }
}

impl convert::From<*const Namespace> for Object {
    fn from(ptr: *const Namespace) -> Self {
        let ptr = ptr as u64;
        Object(ObjectTag::Namespace.tag(ptr))
    }
}

impl<T> convert::From<*mut T> for Object
where
    Object: convert::From<*const T>,
{
    fn from(ptr: *mut T) -> Self {
        Object::from(ptr as *const T)
    }
}

impl convert::From<bool> for Object {
    fn from(b: bool) -> Self {
        if b {
            Object::t()
        } else {
            Object::nil()
        }
    }
}

impl convert::From<f64> for Object {
    fn from(num: f64) -> Self {
        Object(f64::to_bits(num))
    }
}

impl convert::From<i32> for Object {
    fn from(num: i32) -> Self {
        Object(ObjectTag::Integer.tag(num as u64))
    }
}

impl cmp::PartialEq for Object {
    fn eq(&self, other: &Object) -> bool {
        self.0 == other.0
    }
}

impl cmp::Eq for Object {}

#[cfg(test)]
mod test {
    use types::*;
    #[test]
    fn print_a_number() {
        let one = Object::from(1.0);
        println!("{}", one);
    }
}
