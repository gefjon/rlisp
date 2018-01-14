use std::fmt;
use std::cmp::{Eq, PartialEq};
use std::str::FromStr;
use result::*;
use types::*;
use gc::{GarbageCollected, GcMark};

#[derive(Clone)]
pub struct Symbol {
    pub name: String,
    pub val: Option<Object>,
    pub gc_marking: GcMark,
}

impl FromStr for Symbol {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        Ok(Symbol {
            name: String::from(s),
            val: None,
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
        if let Some(obj) = self.val {
            obj.gc_mark(mark);
        }
    }
}

impl Symbol {
    pub fn from_string(sym: String) -> Self {
        Symbol {
            name: sym,
            val: None,
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
