use std::{fmt, mem};
use std::cmp::{Eq, PartialEq};
use std::str::FromStr;
use result::*;
use types::*;
use gc::{GarbageCollected, GcMark};
use std::boxed::Box;
use std::default::Default;

pub struct Symbol {
    pub name: String,
    pub val: Binding,
    pub gc_marking: GcMark,
}

pub struct Binding {
    bind: Option<Object>,
    prev: Option<Box<Binding>>,
}

impl Binding {
    pub fn push(&mut self, val: Object) {
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
    pub fn pop(&mut self) -> Option<Object> {
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
}

impl Default for Binding {
    fn default() -> Self {
        Self {
            bind: None,
            prev: None,
        }
    }
}

impl FromStr for Symbol {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        Ok(Symbol {
            name: String::from(s),
            val: Binding::default(),
            gc_marking: 0,
        })
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
        if let Some(val) = self.evaluate() {
            val.gc_mark(mark);
        }
    }
}

impl Symbol {
    pub fn evaluate(&self) -> Option<Object> {
        if let Binding {
            bind: Some(val), ..
        } = self.val
        {
            Some(val)
        } else {
            None
        }
    }
    pub fn from_string(sym: String) -> Self {
        Symbol {
            name: sym,
            val: Binding::default(),
            gc_marking: 0,
        }
    }
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        ::std::ptr::eq(self, other)
    }
}

impl Eq for Symbol {}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
