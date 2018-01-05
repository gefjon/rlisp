use std::rc::Rc;
use std::fmt;
use std::cmp::{PartialEq, Eq};
use types::Object;
use std::convert;

#[derive(Clone)]
pub struct Symbol {
    sym: Rc<String>,
}

impl Symbol {
    pub fn from_str(sym: &str) -> Self {
        Symbol {
            sym: Rc::new(String::from(sym)),
        }
    }
    pub fn from_string(sym: String) -> Self {
        Symbol {
            sym: Rc::new(sym),
        }
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
