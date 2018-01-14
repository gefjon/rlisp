use lisp;
use result::*;
use types::*;
use gc::{GarbageCollected, GcMark};
use std::fmt;

pub struct RlispFunc {
    pub arglist: Object,
    pub body: FunctionBody,
    gc_marking: GcMark,
    name: Option<Object>,
}

pub enum FunctionBody {
    RustFn(Box<Fn(&mut lisp::Lisp) -> Result<Object>>),
    LispFn(Vec<Object>),
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
