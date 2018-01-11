use lisp;
use result::*;
use types::*;

pub trait Evaluator: lisp::Symbols {
    fn evaluate(&mut self, input: Object) -> Result<Object> {
        match input {
            Object::Sym(s) => self.eval_symbol(s),
            Object::Cons(c) => self.eval_list(c),
            Object::Nil | Object::String(_) | Object::Num(_) => Ok(input),
        }
    }
    fn eval_symbol(&mut self, s: *const Symbol) -> Result<Object> {
        if let Some(obj) = unsafe { (*s).val } {
            Ok(obj)
        } else {
            Err(ErrorKind::UnboundSymbol.into())
        }
    }
    fn eval_list(&mut self, _c: *const ConsCell) -> Result<Object> {
        unimplemented!()
    }
}

impl Evaluator for lisp::Lisp {}
