use std::convert;
use std::fmt;
use gc::{GarbageCollected, GcMark};

pub struct RlispString {
    pub gc_marking: GcMark,
    pub val: String,
}

impl GarbageCollected for RlispString {
    fn my_marking(&self) -> &GcMark {
        &self.gc_marking
    }
    fn my_marking_mut(&mut self) -> &mut GcMark {
        &mut self.gc_marking
    }
    fn gc_mark_children(&mut self, _mark: GcMark) {}
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
