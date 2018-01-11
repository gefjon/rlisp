use std::rc::Rc;
use std::fmt;
use std::cmp::{Eq, PartialEq};
use std::str::FromStr;
use result::*;

#[derive(Clone)]
pub struct Symbol {
    sym: Rc<String>,
}

impl FromStr for Symbol {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        Ok(Symbol {
            sym: Rc::new(String::from(s)),
        })
    }
}

impl Symbol {
    pub fn from_string(sym: String) -> Self {
        Symbol { sym: Rc::new(sym) }
    }
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.sym, &other.sym)
    }
}

impl Eq for Symbol {}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", *(self.sym))
    }
}
