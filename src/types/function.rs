/*
This module stores functions, but they are constructed by `builtins`
and evaluated by `evaluator`.
*/

use types::*;
use gc::{GarbageCollected, GcMark};
use std::fmt;
use builtins;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ArgType {
    Mandatory,
    Optional,
    Rest,
}

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
    fn gc_mark_children(&mut self, mark: GcMark) {
        if let FunctionBody::LispFn(ref vector) = self.body {
            for obj in vector {
                obj.gc_mark(mark);
            }
        }
        if let Some(arglist) = self.arglist {
            arglist.gc_mark(mark);
        }
        if let Some(name) = self.name {
            name.gc_mark(mark);
        }
    }
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

impl fmt::Debug for RlispFunc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[ function {} ({}) -> {:?} ]",
            self.name.unwrap_or(Object::nil()),
            self.arglist.unwrap_or(Object::nil()),
            self.body
        )
    }
}

impl fmt::Debug for FunctionBody {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FunctionBody::RustFn(_) => write!(f, "COMPILED BUILTIN"),
            FunctionBody::LispFn(ref vector) => {
                for el in vector {
                    write!(f, "{:?}", el)?;
                }
                Ok(())
            }
        }
    }
}
