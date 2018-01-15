/*
This module contains types and methods for raw cons
cells. Higher-level List methods are contained in the module list.
*/

use std::fmt;
use list;
use super::Object;
use gc::{GarbageCollected, GcMark};

#[derive(Clone)]
pub struct ConsCell {
    pub car: Object,
    pub cdr: Object,
    gc_marking: GcMark,
}

impl ConsCell {
    pub fn new(car: Object, cdr: Object) -> Self {
        Self {
            car: car,
            cdr: cdr,
            gc_marking: 0,
        }
    }
}

impl GarbageCollected for ConsCell {
    fn my_marking(&self) -> &GcMark {
        &self.gc_marking
    }
    fn my_marking_mut(&mut self) -> &mut GcMark {
        &mut self.gc_marking
    }
    fn gc_mark_children(&mut self, mark: GcMark) {
        self.car.gc_mark(mark);
        self.cdr.gc_mark(mark);
    }
}

impl fmt::Display for ConsCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use list::ConsIteratorResult::*;

        write!(f, "(")?;
        let mut iter = list::iter(self);

        if let More(obj) = iter.improper_next() {
            // A list will always have a first item, so we don't need to check
            // for Final in this one
            write!(f, "{}", obj)?;
        }

        'iter: loop {
            let res = iter.improper_next();
            if let More(obj) = res {
                write!(f, " {}", obj)?;
            } else if let Final(Some(obj)) = res {
                write!(f, " . {}", obj)?;
                break 'iter;
            } else {
                break 'iter;
            }
        }

        write!(f, ")")?;
        Ok(())
    }
}
