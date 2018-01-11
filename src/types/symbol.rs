use std::fmt;
use std::cmp::{Eq, PartialEq};
use std::str::FromStr;
use result::*;
use types::*;

#[derive(Clone)]
pub struct Symbol {
    pub name: String,
    pub val: Option<Object>,
}

impl FromStr for Symbol {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        Ok(Symbol {
            name: String::from(s),
            val: None,
        })
    }
}

impl Symbol {
    pub fn from_string(sym: String) -> Self {
        Symbol {
            name: sym,
            val: None,
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
