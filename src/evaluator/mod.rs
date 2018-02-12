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
use types::conversions::*;
use symbols_table::SymbolLookup;

pub trait Evaluator
    : SymbolLookup + lisp::stack_storage::Stack + gc::GarbageCollector + list::ListOps
    {
    fn evaluate(&mut self, input: Object) -> Object {
        debug!("evaluating {}", input);
        debug!(
            "evaluate(): pushing {} to the stack so it doesn't get gc'd",
            input
        );
        self.push(input); // push `input` to the stack so that the gc doesn't get rid of it
        let res = match input.what_type() {
            RlispType::Sym => unsafe { self.get_symbol(<*const Symbol>::from_unchecked(input)) },
            RlispType::Cons => self.eval_list(unsafe { <&ConsCell>::from_unchecked(input) }),
            RlispType::Num
            | RlispType::NatNum
            | RlispType::Integer
            | RlispType::Bool
            | RlispType::String
            | RlispType::Function
            | RlispType::Error
            | RlispType::Namespace => input,
        };
        self.gc_maybe_pass();
        debug!("{} evaluated to {}", input, res);
        let _popped = self.pop();
        debug!(
            "evaluate(): popped {} from the stack as we have finished evaluating it",
            _popped
        );
        debug_assert!(_popped == input);
        res
    }
    fn eval_list(&mut self, c: *const ConsCell) -> Object {
        // Evaluating a list entails treating the car as a function
        // and calling it with the rest of the list as arguments.
        // Future improvement: push NumArgs to the stack to allow for
        // &optional and &rest args

        let &ConsCell { car, cdr, .. } = unsafe { &(*c) };
        let car = self.evaluate(car);
        let func = into_type_or_error!(self : car => &mut RlispFunc);

        if let FunctionBody::SpecialForm(ref mut func) = func.body {
            debug!("eval_list(): {} is a special form", car);
            let num_args = if let Some(cons) = cdr.maybe_into() {
                let mut iter = list::iter(unsafe { self.list_reverse(cons).into_unchecked() });
                let mut num_args: u32 = 0;
                loop {
                    let res = iter.improper_next();
                    if let list::ConsIteratorResult::Final(Some(_)) = res {
                        let e: Error = ErrorKind::ImproperList.into();
                        let e: RlispError = e.into();
                        return self.alloc(e);
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
            let num_args = if let Some(cons) = cdr.maybe_into() {
                let mut iter = list::iter(unsafe { self.list_reverse(cons).into_unchecked() });
                let mut num_args: u32 = 0;
                loop {
                    let res = iter.improper_next();
                    if let list::ConsIteratorResult::Final(Some(_)) = res {
                        let e: Error = ErrorKind::ImproperList.into();
                        let e: RlispError = e.into();
                        return self.alloc(e);
                    } else if let list::ConsIteratorResult::More(obj) = res {
                        let obj = self.evaluate(obj);
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
            self.put_function_scope_and_call(func)
        }
    }
    fn call_special_form(&mut self, func: &mut RlispSpecialForm) -> Object;
    fn call_rust_func(&mut self, func: &mut RlispBuiltinFunc, n_args: u32) -> Object;
    // These methods are left up to the implementor because
    // `RlispBuiltinFunc`s take an &mut lisp::Lisp, which is not the
    // same as taking an &mut Self
    fn arglist_compat(&mut self, arglist: &ConsCell, n_args: u32) -> Result<bool> {
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
        arglist: Option<&ConsCell>,
        n_args: u32,
    ) -> Object {
        match func.body {
            FunctionBody::LispFn(ref funcb) => {
                if let Some(arglist) = arglist {
                    bubble!(self.get_args_for_lisp_func(arglist, n_args));
                }
                let mut ret = Object::nil();
                for line in funcb {
                    ret = self.evaluate(*line);
                }
                if arglist.is_some() {
                    self.pop_args_from_lisp_func();
                }
                ret
            }
            FunctionBody::RustFn(ref mut funcb) => self.call_rust_func((*funcb).as_mut(), n_args),
            FunctionBody::SpecialForm(_) => unreachable!(),
        }
    }
    fn put_function_scope_and_call(&mut self, func: &mut RlispFunc) -> Object {
        if let Some(ref scope) = func.scope {
            for nmspc in scope {
                self.push_namespace(*nmspc);
            }
        }
        let res = self.funcall(func);
        if let Some(ref scope) = func.scope {
            for _ in scope {
                self.end_scope();
            }
        }
        res
    }
    fn funcall_unchecked(&mut self, func: &mut RlispFunc, n_args: u32) -> Object {
        warn!("Calling a function without checking args!");
        if let FunctionBody::RustFn(ref mut funcb) = func.body {
            self.call_rust_func((*funcb).as_mut(), n_args)
        } else {
            let e: Error = ErrorKind::RequiresArglist.into();
            let e: RlispError = e.into();
            self.alloc(e)
        }
    }
    fn funcall(&mut self, func: &mut RlispFunc) -> Object {
        debug!("calling function {:?}", func);
        let n_args = pop_bubble!(self);
        let n_args: u32 = unsafe { n_args.into_unchecked() };
        debug!("was passed {} args", n_args);
        if let Some(arglist) = func.arglist {
            if let Some(arglist) = arglist.maybe_into() {
                if try_rlisp_err!(self :
                                      self.arglist_compat(arglist, n_args))
                {
                    debug!("#{} is compatible with the arglist {}", n_args, arglist);
                    self.funcall_after_check(func, Some(arglist), n_args)
                } else {
                    debug!(
                        "#{} is not compatible with the arglist {:?}",
                        n_args, arglist
                    );
                    let (min_args, max_args) = try_rlisp_err!(self :
                                           self.acceptable_range(arglist));
                    let e: Error = ErrorKind::WrongArgsCount(n_args, min_args, max_args).into();
                    let e: RlispError = e.into();
                    self.alloc(e)
                }
            } else {
                debug_assert!(arglist == Object::nil());
                if n_args == 0 {
                    self.funcall_after_check(func, None, n_args)
                } else {
                    let e: Error = ErrorKind::WrongArgsCount(n_args, 0, Some(0)).into();
                    let e: RlispError = e.into();
                    self.alloc(e)
                }
            }
        } else {
            self.funcall_unchecked(func, n_args)
        }
    }

    fn acceptable_range(&mut self, arglist: &ConsCell) -> Result<(u32, Option<u32>)> {
        use types::function::ArgType;
        debug!("Checking acceptable range for arglist {}", arglist);
        let mut iter = list::iter(arglist);
        let mut min_args: u32 = 0;
        let mut max_args: Option<u32> = Some(0);
        let mut arg_type = ArgType::Mandatory;
        loop {
            let res = iter.improper_next();
            if let list::ConsIteratorResult::Final(Some(_)) = res {
                return Err(ErrorKind::ImproperList.into());
            } else if let list::ConsIteratorResult::More(obj) = res {
                let sym = <*const Symbol as MaybeFrom<Object>>::maybe_from(obj).unwrap();
                let arg_name: &[u8] = unsafe { &*sym }.as_ref();
                if arg_name == b"&optional" {
                    arg_type = ArgType::Optional;
                } else if arg_name == b"&rest" {
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

    fn get_args_for_lisp_func(&mut self, arglist: &ConsCell, n_args: u32) -> Object {
        use types::function::ArgType;
        debug!("getting arglist {}", arglist);
        // iterate through an arglist, pop()ing off the stack for each
        // item and binding the arg to the pop()ed value.
        let mut iter = list::iter(arglist);
        let mut arg_type = ArgType::Mandatory;
        let mut consumed = 0;
        let mut args = Vec::with_capacity(n_args as _);
        loop {
            let res = iter.improper_next();
            if let ConsIteratorResult::More(sym) = res {
                let sym = into_type_or_error!(self : sym => *const Symbol);
                let arg_name: &[u8] = unsafe { &*sym }.as_ref();
                if arg_name == b"&optional" {
                    debug!("switching to &optional args");
                    arg_type = ArgType::Optional;
                } else if arg_name == b"&rest" {
                    debug!("switching to &rest args");
                    arg_type = ArgType::Rest;
                } else {
                    match arg_type {
                        ArgType::Mandatory => {
                            let arg = pop_bubble!(self);
                            args.push((sym, arg));
                            consumed += 1;
                        }
                        ArgType::Optional => {
                            if consumed < n_args {
                                let arg = pop_bubble!(self);
                                debug!("get_args_for_lisp_func(): popped the arg {}", arg);
                                args.push((sym, arg));
                                consumed += 1;
                            } else {
                                args.push((sym, Object::nil()));
                            }
                        }
                        ArgType::Rest => {
                            let mut head = Object::nil();
                            while consumed < n_args {
                                consumed += 1;
                                let arg = pop_bubble!(self);
                                debug!("get_args_for_lisp_func(): popped the arg {}", arg);
                                let conscell = ConsCell::new(arg, head);
                                head = self.alloc(conscell);
                            }
                            args.push((
                                sym,
                                if head != Object::nil() {
                                    self.list_reverse(unsafe { head.into_unchecked() })
                                } else {
                                    head
                                },
                            ));
                        }
                    }
                }
            } else if let ConsIteratorResult::Final(Some(_)) = res {
                let e: Error = ErrorKind::ImproperList.into();
                let e: RlispError = e.into();
                return self.alloc(e);
            } else {
                break;
            }
        }
        self.new_scope(&args);
        Object::nil()
    }
    fn pop_args_from_lisp_func(&mut self) {
        // This method is called after evaluating a LispFn to unbind
        // the args
        self.end_scope();
    }
}

impl Evaluator for lisp::Lisp {
    fn call_rust_func(&mut self, func: &mut RlispBuiltinFunc, n_args: u32) -> Object {
        debug!("calling a builtin function");
        func(self, n_args)
    }
    fn call_special_form(&mut self, func: &mut RlispSpecialForm) -> Object {
        debug!("calling a special form");
        func(self)
    }
}
