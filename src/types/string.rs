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
    fn gc_mark_children(&mut self, _mark: GcMark) {
        // `RlispString`s don't have any children, so this is a no-op
    }
}

impl fmt::Display for RlispString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // fmt::Display on Object puts quotes around the
        // string. That's not for any particular reason, and if it
        // makes sense in the future the quotes could move here and
        // Object could just straight-up format this value.
        write!(f, "{}", self.val)
    }
}

impl fmt::Debug for RlispString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ string \"{}\" ]", self.val)
    }
}

impl convert::From<String> for RlispString {
    fn from(val: String) -> Self {
        Self { gc_marking: 0, val }
    }
}
