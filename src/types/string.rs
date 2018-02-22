use std::convert;
use std::{fmt, ops, slice, str};
use gc::{GarbageCollected, GcMark};
use types::*;

pub struct RlispString {
    pub gc_marking: GcMark,
    pub len: usize,
    val: u8,
}

impl GarbageCollected for RlispString {
    fn my_marking(&self) -> &GcMark {
        &self.gc_marking
    }
    fn my_marking_mut(&mut self) -> &mut GcMark {
        &mut self.gc_marking
    }
    fn gc_mark_children(&mut self, _mark: GcMark) {
        // `RlispString`s don't have any children, so this is a no-op
    }
}

impl fmt::Display for RlispString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // fmt::Display on Object puts quotes around the
        // string. That's not for any particular reason, and if it
        // makes sense in the future the quotes could move here and
        // Object could just straight-up format this value.
        write!(f, "{}", <Self as AsRef<str>>::as_ref(self))
    }
}

impl fmt::Debug for RlispString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", <Self as AsRef<str>>::as_ref(self))
    }
}

impl ops::Index<u32> for RlispString {
    type Output = u8;
    fn index(&self, index: u32) -> &u8 {
        &<Self as convert::AsRef<[u8]>>::as_ref(self)[index as usize]
    }
}

impl ops::Index<i32> for RlispString {
    type Output = u8;
    fn index(&self, mut index: i32) -> &u8 {
        if index < 0 {
            index += self.len as i32;
        }
        <Self as ops::Index<u32>>::index(self, index as u32)
    }
}

impl convert::AsRef<str> for RlispString {
    fn as_ref(&self) -> &str {
        unsafe {
            let slice = slice::from_raw_parts((&self.val) as _, self.len);
            str::from_utf8_unchecked(slice)
        }
    }
}

impl convert::AsRef<[u8]> for RlispString {
    fn as_ref(&self) -> &[u8] {
        unsafe { slice::from_raw_parts((&self.val) as _, self.len) }
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
