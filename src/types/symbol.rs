use std::{convert, fmt, mem};
use std::cmp::{Eq, PartialEq};
use types::*;
use gc::{GarbageCollected, GcMark};
use std::default::Default;
use std::{slice, str};

pub struct Symbol {
    pub gc_marking: GcMark,
    val: Binding,
    name_len: usize,
    name: u8,
}

impl Symbol {
    pub fn push(&mut self, val: Object) {
        // called when creating a local binding
        self.val.push(val);
    }
    pub fn pop(&mut self) -> Option<Object> {
        // called when ending a local binding
        self.val.pop()
    }
    pub fn reset(&mut self, val: Object) {
        // called by `defvar` and similar
        self.val = Binding::from(val)
    }
    pub fn set(&mut self, val: Object) {
        // called by `setf` and similar
        self.val.set(val);
    }
    pub fn get(&self) -> Option<Object> {
        self.val.get()
    }
}

#[derive(Debug)]
pub struct Binding {
    // Bindings are a singly-linked list. Function calls, `let`, and
    // similar local bindings push to the lists and pop when their
    // scopes end. `defun`, `defvar`, etc. replace the entire list of
    // Bindings. `setf` replaces the head of the list but leaves the
    // rest intact.
    bind: Option<Object>,
    prev: Option<Box<Binding>>,
}

impl Binding {
    // Binding's methods are private so that they can be changed easily.
    fn get(&self) -> Option<Object> {
        self.bind
    }
    fn set(&mut self, val: Object) {
        self.bind = Some(val);
    }
    fn push(&mut self, val: Object) {
        let old_binding = mem::replace(
            self,
            Binding {
                bind: Some(val),
                prev: None,
            },
        );
        let boxed = Box::new(old_binding);
        self.prev = Some(boxed);
    }
    fn pop(&mut self) -> Option<Object> {
        if let Some(mut prev) = mem::replace(&mut self.prev, None) {
            mem::swap(self, &mut *prev);
            if let Binding {
                bind: Some(obj), ..
            } = *prev
            {
                Some(obj)
            } else {
                None
            }
        } else {
            None
        }
    }
    fn gc_mark(&self, mark: GcMark) {
        if let Some(obj) = self.bind {
            obj.gc_mark(mark);
        }
        if let Some(ref prev) = self.prev {
            prev.gc_mark(mark);
        }
    }
}

impl convert::From<Object> for Binding {
    fn from(obj: Object) -> Self {
        Self {
            bind: Some(obj),
            prev: None,
        }
    }
}

impl Default for Binding {
    fn default() -> Self {
        Self {
            bind: None,
            prev: None,
        }
    }
}

impl GarbageCollected for Symbol {
    fn my_marking(&self) -> &GcMark {
        &self.gc_marking
    }
    fn my_marking_mut(&mut self) -> &mut GcMark {
        &mut self.gc_marking
    }
    fn gc_mark_children(&mut self, mark: GcMark) {
        self.val.gc_mark(mark);
    }
}

impl Symbol {}

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
            "[ symbol {} -> {:?} ]",
            <Self as convert::AsRef<str>>::as_ref(self),
            self.val
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
