use lisp;
use result::*;
use types::*;
use list;

pub trait Evaluator: lisp::Symbols + function::EvalFunc {
    fn evaluate(&mut self, input: Object) -> Result<Object> {
        match input {
            Object::Sym(s) => self.eval_symbol(s),
            Object::Cons(c) => self.eval_list(c),
            Object::Nil | Object::String(_) | Object::Num(_) | Object::Function(_) => Ok(input),
        }
    }
    fn eval_symbol(&mut self, s: *const Symbol) -> Result<Object> {
        if let Some(obj) = unsafe { (*s).val } {
            Ok(obj)
        } else {
            Err(ErrorKind::UnboundSymbol.into())
        }
    }
    fn eval_list(&mut self, c: *const ConsCell) -> Result<Object> {
        use list::ConsIteratorResult;
        let mut iter = list::iter(unsafe { &(*c) });
        let func = match iter.improper_next() {
            ConsIteratorResult::More(obj) => {
                if let Some(f) = obj.into_function() {
                    f
                } else {
                    return Err(ErrorKind::NotAFunction.into());
                }
            }
            _ => unreachable!(), // lists always have at least one item
        };
        loop {
            let res = iter.improper_next();
            if let list::ConsIteratorResult::Final(Some(_)) = res {
                return Err(ErrorKind::ImproperList.into());
            } else if let list::ConsIteratorResult::More(obj) = res {
                self.push(obj);
            } else {
                break;
            }
        }
        self.funcall(func)
    }
}

impl Evaluator for lisp::Lisp {}
