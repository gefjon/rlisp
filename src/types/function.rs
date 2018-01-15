/*
This module stores functions, but they are constructed by `builtins`
and evaluated by `evaluator`.
*/

use types::*;
use gc::{GarbageCollected, GcMark};
use std::fmt;
use builtins;

pub struct RlispFunc {
    // arglist is currently an Option because I haven't bothered to
    // make the builtin macro generate an arglist. That is an
    // important quality-of-life improvement that should be done
    // before too long.

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
