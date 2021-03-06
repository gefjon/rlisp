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

pub mod num;
pub use self::num::RlispNum;

pub mod places;
pub use self::places::Place;

///  Any NaN has these bits set
const NAN_MASK: u64 = 0b111_1111_1111 << 52;

/// `x86_64` pointers always fit in 48 bits; this is used in a `debug_assert`
const _MAX_PTR: u64 = 1 << 48;

const _MAX_IMMEDIATE: u64 = 1 << 32;

/// for type-checking Objects
const OBJECT_TAG_MASK: u64 = 0b1111 << 48;

/// A NaN-boxed Rlisp object, containing either an `f64` or a variant of
/// `ObjectTag`
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
    /// *const ConsCell / *mut ConsCell
    Cons,

    /// *const Symbol / *mut Symbol
    Sym,

    /// *const RlispString / *mut RlispString
    String,

    /// *const RlispFunc / *mut RlispFunc
    Function,

    /// *const RlispError / *mut RlispError
    Error,

    /// *const Namespace / *mut Namespace
    Namespace,

    /// An immediate value; see `ImmediateTag`
    Immediate,

    Place,
}

impl convert::From<ObjectTag> for u64 {
    fn from(t: ObjectTag) -> u64 {
        ((t as u64) << 48)
    }
}

impl ObjectTag {
    /// given a u64 which is strictly less than 1 << 48 (the max x86 pointer),
    /// return its NaNboxed representation
    fn tag(self, ptr: u64) -> u64 {
        debug_assert!(ptr < _MAX_PTR);
        let tagged = u64::from(self) ^ NAN_MASK ^ ptr;
        debug!("tagged the {:?}*\n{:#066b} as\n{:#066b}", self, ptr, tagged);
        tagged
    }

    /// true iff ptr is a NaNboxed self
    fn is_of_type(self, ptr: u64) -> bool {
        let res = !Object::floatp(Object(ptr)) && (ptr & OBJECT_TAG_MASK) == u64::from(self);
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

    /// given a NaNboxed Self, return its raw bits
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

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ImmediateTag {
    Bool,
    Integer,
}

impl convert::From<ImmediateTag> for u64 {
    fn from(t: ImmediateTag) -> u64 {
        ((t as u64) << 32)
    }
}

impl ImmediateTag {
    fn tag(self, val: u64) -> u64 {
        debug_assert!(val < _MAX_IMMEDIATE);
        ObjectTag::Immediate.tag(u64::from(self) ^ val)
    }
    fn is_of_type(self, val: u64) -> bool {
        ObjectTag::Immediate.is_of_type(val) && (val & (u64::from(self))) == (u64::from(self))
    }
    fn untag(self, val: u64) -> u64 {
        debug_assert!(self.is_of_type(val));
        ObjectTag::Immediate.untag(val & !(u64::from(self)))
    }
}

/// These are simple internally-used typenames and are not ever cast
/// to a numeric type or used in tagging or any of that crap
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RlispType {
    Cons,
    Number,
    Integer,
    Float,
    Sym,
    String,
    Function,
    Bool,
    Error,
    Namespace,
    Place,
}

impl RlispType {
    pub fn check_type(self, obj: Object) -> bool {
        match self {
            RlispType::Cons => <*const ConsCell>::is_type_or_place(obj),
            RlispType::Number => RlispNum::is_type_or_place(obj),
            RlispType::Integer => i32::is_type_or_place(obj),
            RlispType::Float => f64::is_type_or_place(obj),
            RlispType::Sym => <*const Symbol>::is_type_or_place(obj),
            RlispType::String => <*const RlispString>::is_type_or_place(obj),
            RlispType::Function => <*const RlispFunc>::is_type_or_place(obj),
            RlispType::Bool => bool::is_type_or_place(obj),
            RlispType::Error => <*const RlispError>::is_type_or_place(obj),
            RlispType::Namespace => <*const Namespace>::is_type_or_place(obj),
            RlispType::Place => {
                let place = unsafe { Place::from_unchecked(obj) };
                self.check_type(*place)
            }
        }
    }
}

impl Object {
    /// The canonical numeric NaN returned by arithmetic ops
    fn the_nan() -> u64 {
        f64::to_bits(::std::f64::NAN)
    }

    /// The logical inverse of floatp ; true iff self is not an f64
    fn is_nanbox(self) -> bool {
        f64::from_bits(self.0).is_nan() && !self.nanp()
    }

    /// true iff self is the numeric NaN which arithmetic ops return
    fn nanp(self) -> bool {
        self.0 == Self::the_nan()
    }

    /// returns the object which the symbol `nil` evauluates to
    pub fn nil() -> Self {
        Object(ImmediateTag::Bool.tag(0))
    }

    /// returns the object which the symbol `t` evaluates to
    pub fn t() -> Self {
        Object(ImmediateTag::Bool.tag(1))
    }

    /// true if self is a bool. note that any object can be cast to
    /// bool, and every object other than `nil` evaluates to true,
    /// but that this method treats only exactly `t` and `nil` as
    /// bools, and returns false for any other Object.
    pub fn boolp(self) -> bool {
        ImmediateTag::Bool.is_of_type(self.0)
    }
    pub fn numberp(self) -> bool {
        self.floatp() || self.integerp()
    }
    pub fn integerp(self) -> bool {
        ImmediateTag::Integer.is_of_type(self.0)
    }
    pub fn symbolp(self) -> bool {
        ObjectTag::Sym.is_of_type(self.0)
    }
    pub fn floatp(self) -> bool {
        !self.is_nanbox()
    }

    /// note that being a cons does not mean being a proper
    /// list. listp is a more expensive (and as yet unimplemented)
    /// operation which involves traversing the list to check that
    /// it is nil-terminated.
    pub fn consp(self) -> bool {
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

    pub fn placep(self) -> bool {
        ObjectTag::Place.is_of_type(self.0)
    }

    /// the logical inverse of casting an Object to bool; true iff
    /// self == Object::nil().
    pub fn nilp(self) -> bool {
        if let Some(b) = bool::maybe_from(self) {
            !b
        } else {
            false
        }
    }

    /// returns the RlispType denoting self's type. Currently pretty
    /// inefficent; O(n) where n is the number of Object
    /// variants. Also not optimized to search more used types
    /// earlier.
    pub fn what_type(self) -> RlispType {
        if self.floatp() {
            RlispType::Float
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
        } else if self.placep() {
            RlispType::Place
        } else {
            unreachable!()
        }
    }

    /// Object could probably implement gc::GarbageCollected, but as
    /// of now it doesn't. Because of that, it instead has this method
    /// and should_dealloc which mimic GarbageCollected::{gc_mark,
    /// should_dealloc} and are called by various types'
    /// gc_mark_children methods.
    pub fn gc_mark(self, marking: ::gc::GcMark) {
        unsafe {
            match self.what_type() {
                RlispType::Number | RlispType::Float | RlispType::Integer | RlispType::Bool => (),
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
                RlispType::Place => (*(Place::from_unchecked(self))).gc_mark(marking),
            }
        }
    }

    /// Object could probably implement gc::GarbageCollected, but as
    /// of now it doesn't. Because of that, it instead has this method
    /// and gc_mark which mimic GarbageCollected::{gc_mark,
    /// should_dealloc} and are called by various types'
    /// gc_mark_children methods.
    pub fn should_dealloc(self, marking: ::gc::GcMark) -> bool {
        unsafe {
            match self.what_type() {
                RlispType::Number | RlispType::Float | RlispType::Integer | RlispType::Bool => {
                    false
                }
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
                RlispType::Place => (*(Place::from_unchecked(self))).should_dealloc(marking),
            }
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            match self.what_type() {
                RlispType::Number => unreachable!(),
                RlispType::Float => write!(f, "{}", f64::from_unchecked(*self)),
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
                RlispType::Place => write!(f, "{}", Place::from_unchecked(*self)),
            }
        }
    }
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            match self.what_type() {
                RlispType::Number => unreachable!(),
                RlispType::Float => write!(f, "{}", f64::from_unchecked(*self)),
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
                RlispType::Place => write!(f, "{:?}", Place::from_unchecked(*self)),
            }
        }
    }
}

impl Default for Object {
    /// the default Object is `t`
    fn default() -> Self {
        Object::t()
    }
}

impl convert::From<Object> for bool {
    /// all values except `nil` and errors evaluate to true
    fn from(obj: Object) -> bool {
        !(obj.nilp() || ObjectTag::Error.is_of_type(obj.0))
    }
}

impl convert::From<RlispNum> for Object {
    fn from(n: RlispNum) -> Self {
        if let Some(i) = i32::maybe_from(n) {
            Object::from(i)
        } else {
            Object::from(f64::from(n))
        }
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
        Object(ImmediateTag::Integer.tag(u64::from(num as u32)))
    }
}

impl<T> convert::From<Option<T>> for Object
where
    Object: convert::From<T>,
{
    fn from(opt: Option<T>) -> Self {
        if let Some(t) = opt {
            Object::from(t)
        } else {
            Object::nil()
        }
    }
}

impl cmp::PartialEq for Object {
    /// eq-comparing Objects is straight-up bitwise equality, which
    /// for numbers is type-specific numeric equality (1.0 != 1), and
    /// for pointers is pointer equality
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
