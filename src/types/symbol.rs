use std::{convert, fmt, ops, slice, str};
use std::cmp::{Eq, PartialEq};
use gc::{GarbageCollected, GcMark};
use types::*;

pub struct Symbol {
    pub gc_marking: GcMark,
    name_len: usize,
    name: u8,
}

impl ops::Index<u32> for Symbol {
    type Output = u8;
    fn index(&self, index: u32) -> &u8 {
        &<Self as convert::AsRef<[u8]>>::as_ref(self)[index as usize]
    }
}

impl ops::Index<i32> for Symbol {
    type Output = u8;
    fn index(&self, mut index: i32) -> &u8 {
        if index < 0 {
            index += self.name_len as i32;
        }
        <Self as ops::Index<u32>>::index(self, index as u32)
    }
}

impl GarbageCollected for Symbol {
    fn my_marking(&self) -> &GcMark {
        &self.gc_marking
    }
    fn my_marking_mut(&mut self) -> &mut GcMark {
        &mut self.gc_marking
    }
    fn gc_mark_children(&mut self, _mark: GcMark) {}
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        ::std::ptr::eq(self, other)
    }
}

impl Eq for Symbol {}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", <Self as convert::AsRef<str>>::as_ref(self))
    }
}

impl fmt::Debug for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[ symbol {} ]",
            <Self as convert::AsRef<str>>::as_ref(self)
        )
    }
}

impl convert::AsRef<str> for Symbol {
    fn as_ref(&self) -> &str {
        unsafe {
            let slice = slice::from_raw_parts((&self.name) as _, self.name_len);
            str::from_utf8_unchecked(slice)
        }
    }
}

impl convert::AsRef<[u8]> for Symbol {
    fn as_ref(&self) -> &[u8] {
        unsafe { slice::from_raw_parts((&self.name) as _, self.name_len) }
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
