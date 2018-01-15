/*
This module stores functions, but they are constructed by `builtins`
and evaluated by `evaluator`.
*/

use types::*;
use gc::{GarbageCollected, GcMark};
use std::fmt;
use builtins;

pub struct RlispFunc {
    // arglist is an Option so that methods can be chained
    // (from_builtin().with_arglist().with_name())

    // TODO: docstrings
    pub arglist: Option<Object>,
    pub body: FunctionBody,
    gc_marking: GcMark,
    name: Option<Object>,
}

pub enum FunctionBody {
    RustFn(Box<builtins::RlispBuiltinFunc>),
    LispFn(Vec<Object>),
}

impl RlispFunc {
    pub fn from_builtin(fun: Box<builtins::RlispBuiltinFunc>) -> Self {
        Self {
            arglist: None,
            body: FunctionBody::RustFn(fun),
            gc_marking: 0,
            name: None,
        }
    }
    pub fn with_arglist(mut self, arglist: Object) -> Self {
        self.arglist = Some(arglist);
        self
    }
    pub fn with_name(mut self, name: Object) -> Self {
        self.name = Some(name);
        self
    }
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
