use std::fmt;
use std::cmp::{Eq, PartialEq};
use std::str::FromStr;
use result::*;
use types::*;
use gc::GcMark;

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

impl Symbol {
    pub fn should_dealloc(&self, current_marking: GcMark) -> bool {
        self.gc_marking != current_marking
    }
    pub fn gc_mark(&mut self, mark: GcMark) {
        if self.gc_marking != mark {
            self.gc_marking = mark;
            if let Some(obj) = self.val {
                obj.gc_mark(mark);
            }
        }
    }
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
