use lisp;
use result::*;
use types::*;
use gc::{GarbageCollected, GcMark};
use std::fmt;

pub struct RlispFunc {
    gc_marking: GcMark,
    name: Option<Object>,
}

impl GarbageCollected for RlispFunc {
    fn my_marking(&self) -> &GcMark {
        &self.gc_marking
    }
    fn my_marking_mut(&mut self) -> &mut GcMark {
        &mut self.gc_marking
    }
    fn gc_mark_children(&mut self, _mark: GcMark) {}
}

impl fmt::Display for RlispFunc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(name) = self.name {
            write!(f, "#'{}", name)
        } else {
            write!(f, "ANONYMOUS FUNCTION")
        }
    }
}

pub trait EvalFunc: lisp::stack_storage::Stack {
    fn funcall(&mut self, _func: &RlispFunc) -> Result<Object> {
        unimplemented!()
    }
}

impl EvalFunc for lisp::Lisp {}
