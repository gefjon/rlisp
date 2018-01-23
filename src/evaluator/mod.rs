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
use builtins::{RlispBuiltinFunc, RlispSpecialForm};
use gc;

pub trait Evaluator
    : lisp::Symbols + lisp::stack_storage::Stack + gc::GarbageCollector + list::ListOps
    {
    fn evaluate(&mut self, input: Object) -> Result<Object> {
        debug!("evaluating {}", input);
        debug!(
            "evaluate(): pushing {} to the stack so it doesn't get gc'd",
            input
        );
        self.push(input); // push `input` to the stack so that the gc doesn't get rid of it
        let res = match input {
            Object::Sym(s) => self.eval_symbol(s),
            Object::Cons(c) => self.eval_list(c),
            Object::Bool(_) | Object::String(_) | Object::Num(_) | Object::Function(_) => Ok(input), // the majority of types evaluate to themselves
        };
        self.gc_maybe_pass();
        if res.is_err() {
            info!("evaluation errored; cleaning stack");
            self.clean_stack();
        }
        if let Ok(obj) = res {
            debug!("{} evaluated to {}", input, obj);
        }
        let _popped = self.pop()?;
        debug!(
            "evaluate(): popped {} from the stack as we have finished evaluating it",
            _popped
        );
        debug_assert!(_popped == input);
        res
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
        let &ConsCell { car, cdr, .. } = unsafe { &(*c) };
        let func = self.evaluate(car)?.into_function_or_error()?;
        if let FunctionBody::SpecialForm(ref mut func) = func.body {
            let num_args = if let Some(cons) = cdr.into_cons() {
                let mut iter = list::iter(self.list_reverse(cons).into_cons_or_error()?);
                let mut num_args: usize = 0;
                loop {
                    let res = iter.improper_next();
                    if let list::ConsIteratorResult::Final(Some(_)) = res {
                        return Err(ErrorKind::ImproperList.into());
                    } else if let list::ConsIteratorResult::More(obj) = res {
                        num_args += 1;
                        debug!("eval_list(): pushing {} as an argument", obj);
                        self.push(obj);
                    } else {
                        break;
                    }
                }
                num_args
            } else {
                0
            };
            debug!("eval_list(): pushing {} as num_args", num_args);
            self.push(Object::from(num_args));
            self.call_special_form((*func).as_mut())
        } else {
            let num_args = if let Some(cons) = cdr.into_cons() {
                let mut iter = list::iter(self.list_reverse(cons).into_cons_or_error()?);
                let mut num_args: usize = 0;
                loop {
                    let res = iter.improper_next();
                    if let list::ConsIteratorResult::Final(Some(_)) = res {
                        return Err(ErrorKind::ImproperList.into());
                    } else if let list::ConsIteratorResult::More(obj) = res {
                        let obj = self.evaluate(obj)?;
                        num_args += 1;
                        debug!("eval_list(): pushing {} as an argument", obj);
                        self.push(obj);
                    } else {
                        break;
                    }
                }
                num_args
            } else {
                0
            };
            debug!("eval_list(): pushing {} as num_args", num_args);
            self.push(Object::from(num_args));
            self.funcall(func)
        }
    }
    fn call_special_form(&mut self, func: &mut RlispSpecialForm) -> Result<Object>;
    fn call_rust_func(&mut self, func: &mut RlispBuiltinFunc, n_args: usize) -> Result<Object>;
    // These methods are left up to the implementor because
    // `RlispBuiltinFunc`s take an &mut lisp::Lisp, which is not the
    // same as taking an &mut Self
    fn arglist_compat(&mut self, arglist: &ConsCell, n_args: usize) -> Result<bool> {
        let (min_args, max_args) = self.acceptable_range(arglist)?;
        if let Some(ma) = max_args {
            Ok((n_args >= min_args) && (n_args <= ma))
        } else {
            Ok(n_args >= min_args)
        }
    }
    fn funcall_after_check(
        &mut self,
        func: &mut RlispFunc,
        arglist: &ConsCell,
        n_args: usize,
    ) -> Result<Object> {
        match func.body {
            FunctionBody::LispFn(ref funcb) => {
                self.get_args_for_lisp_func(arglist, n_args)?;
                let mut ret = Object::nil();
                for line in funcb {
                    ret = self.evaluate(*line)?;
                }
                self.pop_args_from_lisp_func(arglist)?;
                Ok(ret)
            }
            FunctionBody::RustFn(ref mut funcb) => self.call_rust_func((*funcb).as_mut(), n_args),
            FunctionBody::SpecialForm(_) => unreachable!(),
        }
    }
    fn funcall_unchecked(&mut self, func: &mut RlispFunc, n_args: usize) -> Result<Object> {
        warn!("Calling a function without checking args!");
        if let FunctionBody::RustFn(ref mut funcb) = func.body {
            self.call_rust_func((*funcb).as_mut(), n_args)
        } else {
            Err(ErrorKind::RequiresArglist.into())
        }
    }
    fn funcall(&mut self, func: &mut RlispFunc) -> Result<Object> {
        debug!("calling function {:?}", func);
        let n_args = unsafe { self.pop()?.into_usize_unchecked() };
        debug!("was passed {} args", n_args);
        if let Some(arglist) = func.arglist {
            let arglist = arglist.into_cons_or_error()?;
            if self.arglist_compat(arglist, n_args)? {
                debug!("#{} is compatible with the arglist {}", n_args, arglist);
                self.funcall_after_check(func, arglist, n_args)
            } else {
                debug!(
                    "#{} is not compatible with the arglist {:?}",
                    n_args, arglist
                );
                let (min_args, max_args) = self.acceptable_range(arglist)?;
                Err(ErrorKind::WrongArgsCount(n_args, min_args, max_args).into())
            }
        } else {
            self.funcall_unchecked(func, n_args)
        }
    }

    fn acceptable_range(&mut self, arglist: &ConsCell) -> Result<(usize, Option<usize>)> {
        use types::function::ArgType;
        debug!("Checking acceptable range for arglist {}", arglist);
        let mut iter = list::iter(arglist);
        let mut min_args: usize = 0;
        let mut max_args: Option<usize> = Some(0);
        let mut arg_type = ArgType::Mandatory;
        loop {
            let res = iter.improper_next();
            if let list::ConsIteratorResult::Final(Some(_)) = res {
                return Err(ErrorKind::ImproperList.into());
            } else if let list::ConsIteratorResult::More(obj) = res {
                if obj == self.intern("&optional") {
                    arg_type = ArgType::Optional;
                } else if obj == self.intern("&rest") {
                    arg_type = ArgType::Rest;
                } else {
                    match arg_type {
                        ArgType::Mandatory => {
                            min_args += 1;
                            if let Some(ref mut ma) = max_args {
                                *ma += 1;
                            }
                        }
                        ArgType::Optional => {
                            if let Some(ref mut ma) = max_args {
                                *ma += 1;
                            }
                        }
                        ArgType::Rest => {
                            max_args = None;
                        }
                    }
                }
            } else {
                break;
            }
        }
        debug!("Got acceptable range [{}, {:?}]", min_args, max_args);
        Ok((min_args, max_args))
    }

    fn get_args_for_lisp_func(&mut self, arglist: &ConsCell, n_args: usize) -> Result<()> {
        use types::function::ArgType;
        debug!("getting arglist {}", arglist);
        // iterate through an arglist, pop()ing off the stack for each
        // item and binding the arg to the pop()ed value.
        let mut iter = list::iter(arglist);
        let mut arg_type = ArgType::Mandatory;
        let mut consumed = 0;
        loop {
            let res = iter.improper_next();
            if let ConsIteratorResult::More(sym) = res {
                if sym == self.intern("&optional") {
                    debug!("switching to &optional args");
                    arg_type = ArgType::Optional;
                } else if sym == self.intern("&rest") {
                    debug!("switching to &rest args");
                    arg_type = ArgType::Rest;
                } else {
                    let sym = sym.into_symbol_mut_or_error()?;
                    debug!("trying to get arg for symbol {:?}", sym);
                    match arg_type {
                        ArgType::Mandatory => {
                            let arg = self.pop()?;
                            debug!("get_args_for_lisp_func(): popped the arg {}", arg);
                            sym.push(arg);
                            consumed += 1;
                        }
                        ArgType::Optional => {
                            if consumed < n_args {
                                let arg = self.pop()?;
                                debug!("get_args_for_lisp_func(): popped the arg {}", arg);
                                sym.push(arg);
                                consumed += 1;
                            } else {
                                sym.push(Object::nil());
                            }
                        }
                        ArgType::Rest => {
                            let mut head = Object::nil();
                            while consumed < n_args {
                                consumed += 1;
                                let arg = self.pop()?;
                                debug!("get_args_for_lisp_func(): popped the arg {}", arg);
                                let conscell = ConsCell::new(arg, head);
                                head = self.alloc(conscell);
                            }
                            sym.push(if head != Object::nil() {
                                self.list_reverse(head.into_cons_or_error()?)
                            } else {
                                head
                            });
                        }
                    }
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
        debug!("cleaning up after arglist {:?}", arglist);
        let mut iter = list::iter(arglist);
        loop {
            let res = iter.improper_next();
            if let ConsIteratorResult::More(sym) = res {
                if (sym != self.intern("&optional")) && (sym != self.intern("&rest")) {
                    if let Some(sym) = sym.into_symbol_mut() {
                        sym.pop();
                    } else {
                        return Err(ErrorKind::WrongType(RlispType::Sym, sym.what_type()).into());
                    }
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
    fn call_rust_func(&mut self, func: &mut RlispBuiltinFunc, n_args: usize) -> Result<Object> {
        debug!("calling a builtin function");
        func(self, n_args)
    }
    fn call_special_form(&mut self, func: &mut RlispSpecialForm) -> Result<Object> {
        debug!("calling a special form");
        func(self)
    }
}
