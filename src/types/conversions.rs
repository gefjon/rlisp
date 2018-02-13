use types::*;

pub trait MaybeFrom<T>: Sized {
    fn maybe_from(t: T) -> Option<Self>;
}

pub trait FromObject: MaybeFrom<Object> {
    fn rlisp_type() -> RlispType;
    fn is_type(obj: Object) -> bool {
        Self::maybe_from(obj).is_some()
    }
}

pub trait FromUnchecked<T> {
    unsafe fn from_unchecked(obj: T) -> Self;
}

impl<O, T: MaybeFrom<O>> FromUnchecked<O> for T
where
    T: MaybeFrom<O>,
    O: Copy,
    O: ::std::fmt::Debug,
{
    unsafe fn from_unchecked(o: O) -> T {
        if let Some(t) = T::maybe_from(o) {
            t
        } else {
            panic!("FromUnchecked failed converting {:?}", o)
        }
    }
}

pub trait MaybeInto<T>: Sized {
    fn maybe_into(self) -> Option<T>;
}

impl<O, T: MaybeFrom<O>> MaybeInto<T> for O {
    fn maybe_into(self) -> Option<T> {
        <T as MaybeFrom<O>>::maybe_from(self)
    }
}

pub trait IntoUnchecked<T> {
    unsafe fn into_unchecked(self) -> T;
}

impl<T, O> IntoUnchecked<T> for O
where
    O: MaybeInto<T>,
    O: ::std::fmt::Debug,
    O: ::std::marker::Copy,
{
    unsafe fn into_unchecked(self) -> T {
        if let Some(t) = self.maybe_into() {
            t
        } else {
            panic!("MaybeInto failed converting {:?}", self)
        }
    }
}

impl MaybeFrom<Object> for &'static ConsCell {
    fn maybe_from(obj: Object) -> Option<&'static ConsCell> {
        if obj.consp() {
            let ptr = ObjectTag::Cons.untag(obj.0);
            Some(unsafe { &*(ptr as *const ConsCell) })
        } else {
            None
        }
    }
}

impl FromObject for &'static ConsCell {
    fn rlisp_type() -> RlispType {
        RlispType::Cons
    }
}

impl MaybeFrom<Object> for *const ConsCell {
    fn maybe_from(obj: Object) -> Option<Self> {
        if obj.consp() {
            let ptr = ObjectTag::Cons.untag(obj.0);
            Some(ptr as *const ConsCell)
        } else {
            None
        }
    }
}

impl FromObject for *const ConsCell {
    fn rlisp_type() -> RlispType {
        RlispType::Cons
    }
}

impl MaybeFrom<Object> for &'static mut ConsCell {
    fn maybe_from(obj: Object) -> Option<&'static mut ConsCell> {
        if obj.consp() {
            let ptr = ObjectTag::Cons.untag(obj.0);
            Some(unsafe { &mut *(ptr as *mut ConsCell) })
        } else {
            None
        }
    }
}

impl FromObject for &'static mut ConsCell {
    fn rlisp_type() -> RlispType {
        RlispType::Cons
    }
}

impl MaybeFrom<Object> for f64 {
    fn maybe_from(obj: Object) -> Option<f64> {
        if obj.numberp() {
            Some(f64::from_bits(obj.0))
        } else {
            None
        }
    }
}

impl FromObject for f64 {
    fn rlisp_type() -> RlispType {
        RlispType::Num
    }
}

impl MaybeFrom<Object> for &'static Symbol {
    fn maybe_from(obj: Object) -> Option<Self> {
        if obj.symbolp() {
            let ptr = ObjectTag::Sym.untag(obj.0);
            Some(unsafe { &*(ptr as *const Symbol) })
        } else {
            None
        }
    }
}

impl FromObject for &'static Symbol {
    fn rlisp_type() -> RlispType {
        RlispType::Sym
    }
}

impl MaybeFrom<Object> for &'static mut Symbol {
    fn maybe_from(obj: Object) -> Option<Self> {
        if obj.symbolp() {
            let ptr = ObjectTag::Sym.untag(obj.0);
            Some(unsafe { &mut *(ptr as *mut Symbol) })
        } else {
            None
        }
    }
}

impl FromObject for &'static mut Symbol {
    fn rlisp_type() -> RlispType {
        RlispType::Sym
    }
}

impl MaybeFrom<Object> for *const Symbol {
    fn maybe_from(obj: Object) -> Option<Self> {
        if obj.symbolp() {
            let ptr = ObjectTag::Sym.untag(obj.0);
            Some(ptr as *const Symbol)
        } else {
            None
        }
    }
}

impl FromObject for *const Symbol {
    fn rlisp_type() -> RlispType {
        RlispType::Sym
    }
}

impl MaybeFrom<Object> for &'static RlispString {
    fn maybe_from(obj: Object) -> Option<Self> {
        if obj.stringp() {
            let ptr = ObjectTag::String.untag(obj.0);
            Some(unsafe { &*(ptr as *const RlispString) })
        } else {
            None
        }
    }
}

impl FromObject for &'static RlispString {
    fn rlisp_type() -> RlispType {
        RlispType::String
    }
}

impl MaybeFrom<Object> for *const RlispString {
    fn maybe_from(obj: Object) -> Option<Self> {
        if obj.stringp() {
            let ptr = ObjectTag::String.untag(obj.0);
            Some(ptr as *const RlispString)
        } else {
            None
        }
    }
}

impl FromObject for *const RlispString {
    fn rlisp_type() -> RlispType {
        RlispType::String
    }
}

impl MaybeFrom<Object> for &'static mut RlispString {
    fn maybe_from(obj: Object) -> Option<Self> {
        if obj.stringp() {
            let ptr = ObjectTag::String.untag(obj.0);
            Some(unsafe { &mut *(ptr as *mut RlispString) })
        } else {
            None
        }
    }
}

impl FromObject for &'static mut RlispString {
    fn rlisp_type() -> RlispType {
        RlispType::String
    }
}

impl MaybeFrom<Object> for i32 {
    fn maybe_from(obj: Object) -> Option<Self> {
        if obj.integerp() {
            let val = ObjectTag::Integer.untag(obj.0);
            Some(val as i32)
        } else {
            None
        }
    }
}

impl FromObject for i32 {
    fn rlisp_type() -> RlispType {
        RlispType::Integer
    }
}

impl MaybeFrom<Object> for &'static mut RlispFunc {
    fn maybe_from(obj: Object) -> Option<Self> {
        if obj.functionp() {
            let ptr = ObjectTag::Function.untag(obj.0);
            Some(unsafe { &mut *(ptr as *mut RlispFunc) })
        } else {
            None
        }
    }
}

impl FromObject for &'static mut RlispFunc {
    fn rlisp_type() -> RlispType {
        RlispType::Function
    }
}

impl MaybeFrom<Object> for *const RlispFunc {
    fn maybe_from(obj: Object) -> Option<Self> {
        if obj.functionp() {
            let ptr = ObjectTag::Function.untag(obj.0);
            Some(ptr as *const RlispFunc)
        } else {
            None
        }
    }
}

impl FromObject for *const RlispFunc {
    fn rlisp_type() -> RlispType {
        RlispType::Function
    }
}

impl MaybeFrom<Object> for &'static RlispFunc {
    fn maybe_from(obj: Object) -> Option<Self> {
        if obj.functionp() {
            let ptr = ObjectTag::Function.untag(obj.0);
            Some(unsafe { &*(ptr as *const RlispFunc) })
        } else {
            None
        }
    }
}

impl FromObject for &'static RlispFunc {
    fn rlisp_type() -> RlispType {
        RlispType::Function
    }
}

impl MaybeFrom<Object> for &'static RlispError {
    fn maybe_from(obj: Object) -> Option<Self> {
        if obj.errorp() {
            let ptr = ObjectTag::Error.untag(obj.0);
            Some(unsafe { &*(ptr as *const RlispError) })
        } else {
            None
        }
    }
}

impl FromObject for &'static RlispError {
    fn rlisp_type() -> RlispType {
        RlispType::Error
    }
}

impl MaybeFrom<Object> for *const RlispError {
    fn maybe_from(obj: Object) -> Option<Self> {
        if obj.errorp() {
            let ptr = ObjectTag::Error.untag(obj.0);
            Some(ptr as *const RlispError)
        } else {
            None
        }
    }
}

impl FromObject for *const RlispError {
    fn rlisp_type() -> RlispType {
        RlispType::Error
    }
}

impl MaybeFrom<Object> for &'static mut RlispError {
    fn maybe_from(obj: Object) -> Option<Self> {
        if obj.errorp() {
            let ptr = ObjectTag::Error.untag(obj.0);
            Some(unsafe { &mut *(ptr as *mut RlispError) })
        } else {
            None
        }
    }
}

impl FromObject for &'static mut RlispError {
    fn rlisp_type() -> RlispType {
        RlispType::Error
    }
}

impl MaybeFrom<Object> for bool {
    fn maybe_from(obj: Object) -> Option<Self> {
        if obj.boolp() {
            Some(ObjectTag::Bool.untag(obj.0) != 0)
        } else {
            None
        }
    }
}

impl FromObject for bool {
    fn rlisp_type() -> RlispType {
        RlispType::Bool
    }
}

impl MaybeFrom<Object> for *mut Namespace {
    fn maybe_from(obj: Object) -> Option<Self> {
        if obj.namespacep() {
            let ptr = ObjectTag::Namespace.untag(obj.0);
            Some(ptr as *mut Namespace)
        } else {
            None
        }
    }
}

impl FromObject for *mut Namespace {
    fn rlisp_type() -> RlispType {
        RlispType::Namespace
    }
}

impl MaybeFrom<Object> for *const Namespace {
    fn maybe_from(obj: Object) -> Option<Self> {
        if obj.namespacep() {
            let ptr = ObjectTag::Namespace.untag(obj.0);
            Some(ptr as *const Namespace)
        } else {
            None
        }
    }
}

impl FromObject for *const Namespace {
    fn rlisp_type() -> RlispType {
        RlispType::Namespace
    }
}

impl MaybeFrom<Object> for &'static mut Namespace {
    fn maybe_from(obj: Object) -> Option<Self> {
        if let Some(ptr) = <*mut Namespace>::maybe_from(obj) {
            Some(unsafe { &mut *ptr })
        } else {
            None
        }
    }
}

impl FromObject for &'static mut Namespace {
    fn rlisp_type() -> RlispType {
        RlispType::Namespace
    }
}

impl MaybeFrom<Object> for &'static Namespace {
    fn maybe_from(obj: Object) -> Option<Self> {
        if let Some(ptr) = <*mut Namespace>::maybe_from(obj) {
            Some(unsafe { &*ptr })
        } else {
            None
        }
    }
}

impl FromObject for &'static Namespace {
    fn rlisp_type() -> RlispType {
        RlispType::Namespace
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::ptr;
    #[test]
    fn floats() {
        let one_and_a_half = Object::from(1.5);
        assert_eq!(f64::maybe_from(one_and_a_half), Some(1.5));
        assert!(<&ConsCell>::maybe_from(one_and_a_half).is_none());
    }
    #[test]
    fn integers() {
        let one = Object::from(1);
        assert_eq!(i32::maybe_from(one), Some(1));
        assert!(<&ConsCell>::maybe_from(one).is_none());

        let many = Object::from(::std::i32::MAX);
        assert_eq!(i32::maybe_from(many), Some(::std::i32::MAX));
        assert!(<&Namespace>::maybe_from(many).is_none());
    }
    #[test]
    fn pointers() {
        let a_pointer = 0xdead_beef as *const ConsCell;
        let obj = Object::from(a_pointer);
        assert!(f64::maybe_from(obj).is_none());
        assert!(ptr::eq(
            <*const ConsCell>::maybe_from(obj).unwrap(),
            a_pointer
        ));
        assert!(ptr::eq(
            unsafe { <*const ConsCell>::from_unchecked(obj) },
            a_pointer
        ));
    }
}
