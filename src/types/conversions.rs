use types::*;

/// I'm not sure why this trait isn't in libstd ; it's very convenient
/// and seems like a logical counterpart to `TryFrom`.
///
/// Has a companion method `MaybeInto`
pub trait MaybeFrom<T>: Sized {
    fn maybe_from(t: T) -> Option<Self>;
}

/// Extension to `MaybeFrom` specific for converting Object into types
pub trait FromObject {
    fn rlisp_type() -> RlispType;
    fn is_type(obj: Object) -> bool {
        Self::rlisp_type() == obj.what_type()
    }
    fn is_type_or_place(obj: Object) -> bool {
        Self::is_type(obj) || {
            if let Some(place) = Place::maybe_from(obj) {
                Self::is_type(*place)
            } else {
                false
            }
        }
    }
}

/// Like From, but marked unsafe. In Rlisp, this is used for places
/// where we *know* the type of obj.
///
/// Has a companion method `IntoUnchecked`
pub trait FromUnchecked<T> {
    unsafe fn from_unchecked(obj: T) -> Self;
}

pub trait MaybeInto<T>: Sized {
    fn maybe_into(self) -> Option<T>;
}

pub trait IntoUnchecked<T> {
    unsafe fn into_unchecked(self) -> T;
}

impl<T> MaybeFrom<Object> for T
where
    T: FromUnchecked<Object> + FromObject,
{
    default fn maybe_from(obj: Object) -> Option<T> {
        if <T as FromObject>::is_type(obj) {
            Some(unsafe { T::from_unchecked(obj) })
        } else if let Some(place) = Place::maybe_from(obj) {
            T::maybe_from(*place)
        } else {
            None
        }
    }
}

impl<T, O> IntoUnchecked<T> for O
where
    T: FromUnchecked<O>,
{
    unsafe fn into_unchecked(self) -> T {
        T::from_unchecked(self)
    }
}

impl<T, O> MaybeInto<T> for O
where
    T: MaybeFrom<O>,
{
    fn maybe_into(self) -> Option<T> {
        T::maybe_from(self)
    }
}

impl<T> FromUnchecked<Object> for *const T
where
    *mut T: FromUnchecked<Object>,
{
    unsafe fn from_unchecked(obj: Object) -> *const T {
        <*mut T>::from_unchecked(obj) as *const T
    }
}

impl<T> FromUnchecked<Object> for &'static T
where
    *const T: FromUnchecked<Object>,
{
    unsafe fn from_unchecked(obj: Object) -> &'static T {
        &*(<*const T>::from_unchecked(obj))
    }
}

impl<T> FromUnchecked<Object> for &'static mut T
where
    *mut T: FromUnchecked<Object>,
{
    unsafe fn from_unchecked(obj: Object) -> &'static mut T {
        &mut *(<*mut T>::from_unchecked(obj))
    }
}

impl<T> FromObject for *const T
where
    *mut T: FromObject,
{
    fn rlisp_type() -> RlispType {
        <*mut T>::rlisp_type()
    }
}

impl<T> FromObject for &'static T
where
    *const T: FromObject,
{
    fn rlisp_type() -> RlispType {
        <*const T>::rlisp_type()
    }
}

impl<T> FromObject for &'static mut T
where
    *mut T: FromObject,
{
    fn rlisp_type() -> RlispType {
        <*mut T>::rlisp_type()
    }
}

impl FromUnchecked<Object> for *mut ConsCell {
    unsafe fn from_unchecked(obj: Object) -> *mut ConsCell {
        debug_assert!(obj.consp());
        ObjectTag::Cons.untag(obj.0) as _
    }
}

impl FromObject for *mut ConsCell {
    fn rlisp_type() -> RlispType {
        RlispType::Cons
    }
}

impl FromUnchecked<Object> for f64 {
    unsafe fn from_unchecked(obj: Object) -> f64 {
        debug_assert!(obj.floatp());
        f64::from_bits(obj.0)
    }
}

impl FromObject for f64 {
    fn rlisp_type() -> RlispType {
        RlispType::Float
    }
}

impl FromUnchecked<Object> for *mut Symbol {
    unsafe fn from_unchecked(obj: Object) -> *mut Symbol {
        debug_assert!(obj.symbolp());
        ObjectTag::Sym.untag(obj.0) as _
    }
}

impl FromObject for *mut Symbol {
    fn rlisp_type() -> RlispType {
        RlispType::Sym
    }
}

impl FromUnchecked<Object> for *mut RlispString {
    unsafe fn from_unchecked(obj: Object) -> *mut RlispString {
        debug_assert!(obj.stringp());
        ObjectTag::String.untag(obj.0) as *mut RlispString
    }
}

impl FromObject for *mut RlispString {
    fn rlisp_type() -> RlispType {
        RlispType::String
    }
}

impl FromUnchecked<Object> for i32 {
    unsafe fn from_unchecked(obj: Object) -> i32 {
        debug_assert!(obj.integerp());
        ImmediateTag::Integer.untag(obj.0) as u32 as i32
    }
}

impl FromObject for i32 {
    fn rlisp_type() -> RlispType {
        RlispType::Integer
    }
}

impl FromUnchecked<Object> for *mut RlispFunc {
    unsafe fn from_unchecked(obj: Object) -> *mut RlispFunc {
        debug_assert!(obj.functionp());
        ObjectTag::Function.untag(obj.0) as *mut RlispFunc
    }
}

impl FromObject for *mut RlispFunc {
    fn rlisp_type() -> RlispType {
        RlispType::Function
    }
}

impl FromUnchecked<Object> for *mut RlispError {
    unsafe fn from_unchecked(obj: Object) -> *mut RlispError {
        debug_assert!(obj.errorp());
        ObjectTag::Error.untag(obj.0) as *mut RlispError
    }
}

impl FromObject for *mut RlispError {
    fn rlisp_type() -> RlispType {
        RlispType::Error
    }
}

impl FromUnchecked<Object> for bool {
    unsafe fn from_unchecked(obj: Object) -> bool {
        debug_assert!(obj.boolp());
        ImmediateTag::Bool.untag(obj.0) != 0
    }
}

impl FromObject for bool {
    fn rlisp_type() -> RlispType {
        RlispType::Bool
    }
}

impl FromUnchecked<Object> for *mut Namespace {
    unsafe fn from_unchecked(obj: Object) -> *mut Namespace {
        debug_assert!(obj.namespacep());
        ObjectTag::Namespace.untag(obj.0) as *mut Namespace
    }
}

impl FromObject for *mut Namespace {
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
