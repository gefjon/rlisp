use std::convert;
use std::fmt;
use gc::GcMark;

pub struct RlispString {
    pub gc_marking: GcMark,
    pub val: String,
}

impl RlispString {
    pub fn should_dealloc(&self, current_marking: GcMark) -> bool {
        self.gc_marking != current_marking
    }
    pub fn gc_mark(&mut self, mark: GcMark) {
        self.gc_marking = mark
    }
}

impl fmt::Display for RlispString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl convert::From<String> for RlispString {
    fn from(val: String) -> Self {
        Self { gc_marking: 0, val }
    }
}
