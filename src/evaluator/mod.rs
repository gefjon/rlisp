/*
the `Evaluator` trait does the 'E' part of REPL. its forward-facing
operation is `evaluate`, which is passed an Object, evaluates it, and
returns it.
*/
use lisp;
use result::*;
use types::*;
use list;
use list::ConsIteratorResult;
use types::function::*;
use builtins::RlispBuiltinFunc;

pub trait Evaluator: lisp::Symbols + lisp::stack_storage::Stack {
    fn evaluate(&mut self, input: Object) -> Result<Object> {
        match input {
            Object::Sym(s) => self.eval_symbol(s),
            Object::Cons(c) => self.eval_list(c),
            Object::Bool(_) | Object::String(_) | Object::Num(_) | Object::Function(_) => Ok(input),
        }
    }
    fn eval_symbol(&mut self, s: *const Symbol) -> Result<Object> {
        if let Some(obj) = unsafe { (*s).get() } {
            Ok(obj)
        } else {
            Err(ErrorKind::Unbound.into())
        }
    }
    fn eval_list(&mut self, c: *const ConsCell) -> Result<Object> {
        // Evaluating a list entails treating the car as a function
        // and calling it with the rest of the list as arguments.
        // Future improvement: push NumArgs to the stack to allow for
        // &optional and &rest args
        let mut iter = list::iter(unsafe { &(*c) });
        let func = match iter.improper_next() {
            ConsIteratorResult::More(obj) => {
                if let Some(f) = self.evaluate(obj)?.into_function() {
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
                let obj = self.evaluate(obj)?;
                self.push(obj);
            } else {
                break;
            }
        }
        self.funcall(func)
    }
    fn call_rust_func(&mut self, func: &mut RlispBuiltinFunc) -> Result<Object>;
    // This method is left up to the implementor because
    // `RlispBuiltinFunc`s take an &mut lisp::Lisp, which is not the
    // same as taking an &mut Self

    fn funcall(&mut self, func: &mut RlispFunc) -> Result<Object> {
        match func.body {
            FunctionBody::RustFn(ref mut funcb) => self.call_rust_func((*funcb).as_mut()),
            // builtin functions take their arguments themselves
            // because their arguments are not bound to symbols - the
            // args are still pushed to the stack, but the RustFn
            // pop()s them itself
            FunctionBody::LispFn(ref funcb) => {
                // Future improvement: push NumArgs to the stack to
                // allow for &optional and &rest args
                if let Some(arglist) = func.arglist {
                    if let Some(arglist) = arglist.into_cons() {
                        self.get_args_for_lisp_func(arglist)?;
                        let mut ret = Object::nil();
                        for line in funcb {
                            ret = self.evaluate(*line)?;
                        }
                        self.pop_args_from_lisp_func(arglist)?;
                        Ok(ret)
                    } else {
                        Err(ErrorKind::WrongType(RlispType::Cons, arglist.what_type()).into())
                    }
                } else {
                    Err(ErrorKind::Unbound.into())
                }
            }
        }
    }
    fn get_args_for_lisp_func(&mut self, arglist: &ConsCell) -> Result<()> {
        // iterate through an arglist, pop()ing off the stack for each
        // item and binding the arg to the pop()ed value.

        // Future improvement: push NumArgs to the stack to allow for
        // &optional and &rest args
        let mut iter = list::iter(arglist);
        loop {
            let res = iter.improper_next();
            if let ConsIteratorResult::More(sym) = res {
                if let Some(sym) = sym.into_symbol_mut() {
                    sym.push(self.pop()?);
                } else {
                    return Err(ErrorKind::WrongType(RlispType::Sym, sym.what_type()).into());
                }
            } else if let ConsIteratorResult::Final(Some(_)) = res {
                return Err(ErrorKind::ImproperList.into());
            } else {
                break;
            }
        }
        Ok(())
    }
    fn pop_args_from_lisp_func(&mut self, arglist: &ConsCell) -> Result<()> {
        // This method is called after evaluating a LispFn to unbind
        // the args
        let mut iter = list::iter(arglist);
        loop {
            let res = iter.improper_next();
            if let ConsIteratorResult::More(sym) = res {
                if let Some(sym) = sym.into_symbol_mut() {
                    sym.pop();
                } else {
                    return Err(ErrorKind::WrongType(RlispType::Sym, sym.what_type()).into());
                }
            } else if let ConsIteratorResult::Final(Some(_)) = res {
                return Err(ErrorKind::ImproperList.into());
            } else {
                break;
            }
        }
        Ok(())
    }
}

impl Evaluator for lisp::Lisp {
    fn call_rust_func(&mut self, func: &mut RlispBuiltinFunc) -> Result<Object> {
        func(self)
    }
}
