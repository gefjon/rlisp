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

impl<O, T: MaybeFrom<O>> FromUnchecked<O> for T {
    unsafe fn from_unchecked(o: O) -> T {
        if let Some(t) = T::maybe_from(o) {
            t
        } else {
            panic!("FromUnchecked failed")
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

impl<T, O: MaybeInto<T>> IntoUnchecked<T> for O {
    unsafe fn into_unchecked(self) -> T {
        if let Some(t) = self.maybe_into() {
            t
        } else {
            panic!("MaybeInto failed")
        }
    }
}

impl MaybeFrom<Object> for &'static ConsCell {
    fn maybe_from(obj: Object) -> Option<&'static ConsCell> {
        if let Object::Cons(ptr) = obj {
            Some(unsafe { &(*ptr) })
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

impl MaybeFrom<Object> for f64 {
    fn maybe_from(obj: Object) -> Option<f64> {
        if let Object::Num(num) = obj {
            Some(num)
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

impl MaybeFrom<Object> for &'static mut Symbol {
    fn maybe_from(obj: Object) -> Option<Self> {
        if let Object::Sym(ptr) = obj {
            Some(unsafe { &mut (*(ptr as *mut Symbol)) })
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
        if let Object::Sym(ptr) = obj {
            Some(ptr)
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
        if let Object::String(ptr) = obj {
            Some(unsafe { &(*ptr) })
        } else {
            None
        }
    }
}

impl MaybeFrom<Object> for u32 {
    fn maybe_from(obj: Object) -> Option<Self> {
        if let Object::Num(n) = obj {
            if ::math::natnump(n) {
                return Some(n as _);
            }
        }
        None
    }
}

impl FromObject for u32 {
    fn rlisp_type() -> RlispType {
        RlispType::NatNum
    }
}

impl MaybeFrom<Object> for i32 {
    fn maybe_from(obj: Object) -> Option<Self> {
        if let Object::Num(n) = obj {
            if ::math::integerp(n) {
                return Some(n as _);
            }
        }
        None
    }
}

impl FromObject for i32 {
    fn rlisp_type() -> RlispType {
        RlispType::Integer
    }
}

impl FromObject for &'static RlispString {
    fn rlisp_type() -> RlispType {
        RlispType::String
    }
}

impl MaybeFrom<Object> for &'static mut RlispFunc {
    fn maybe_from(obj: Object) -> Option<Self> {
        if let Object::Function(ptr) = obj {
            Some(unsafe { &mut (*(ptr as *mut RlispFunc)) })
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

impl MaybeFrom<Object> for &'static RlispError {
    fn maybe_from(obj: Object) -> Option<Self> {
        if let Object::Error(ptr) = obj {
            Some(unsafe { &(*ptr) })
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

impl MaybeFrom<Object> for bool {
    fn maybe_from(obj: Object) -> Option<Self> {
        if let Object::Bool(b) = obj {
            Some(b)
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
