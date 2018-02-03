use symbols_table::SymbolLookup;
use result::*;
use types::*;
use types::into_object::*;
use lisp;
use std::boxed::Box;
use lisp::allocate::AllocObject;
use types::conversions::*;

// The macros `special_forms` and `builtin_functions` are the main
// part of this module. See `make_builtins()` and `make_special_forms`
// for their use.  Each function consists of a string name, a list of
// identifiers which will have args bound to them, and a block which
// returns an Object. The block can use the `?` operator, but should
// do so sparingly

pub type RlispBuiltinFunc = FnMut(&mut lisp::Lisp, u32) -> Object;
pub type RlispSpecialForm = FnMut(&mut lisp::Lisp) -> Object;
pub type Arglist = Vec<String>;
pub type Name = String;
pub type RlispBuiltinTuple = (Name, Arglist, Box<RlispBuiltinFunc>);
pub type RlispSpecialForms = Vec<(Name, Arglist, Box<RlispSpecialForm>)>;
pub type RlispBuiltins = Vec<RlispBuiltinTuple>;
pub type RlispBuiltinVars = Vec<(Name, IntoObject)>;

pub fn make_special_forms() -> RlispSpecialForms {
    use evaluator::Evaluator;
    special_forms!{
        l = lisp;
        "cond" (&rest clauses) -> {
            let n_args: u32 = unsafe { pop_bubble!(l).into_unchecked() };
            debug!("called cond with {} args", n_args);
            let mut clauses = Vec::with_capacity(n_args as _);
            for _i in 0..n_args {
                debug!("popping arg {}", _i);
                let arg = pop_bubble!(l);
                debug!("arg {} was {}", _i, arg);
                clauses.push(arg);
            }
            for clause in &clauses {
                let &ConsCell { car, cdr, .. } = into_type_or_error!(l : *clause => &ConsCell);
                if bool::from(l.evaluate(car)) {
                    let &ConsCell { car: cdrcar, .. } =  into_type_or_error!(l : cdr => &ConsCell);
                    return l.evaluate(cdrcar);
                }
            }
            false.into()
        },
        "let" (bindings &rest body) -> {
            let n_args: u32 = unsafe { pop_bubble!(l).into_unchecked() };
            let bindings = pop_bubble!(l);
            let mut body = Vec::with_capacity(n_args as usize - 1);
            for _i in 0..(n_args - 1) {
                let arg = pop_bubble!(l);
                body.push(arg);
            }
            let mut scope = Vec::new();

            #[cfg_attr(feature = "cargo-clippy", allow(explicit_iter_loop))]
            for binding_pair in into_type_or_error!(l : bindings => &ConsCell).into_iter() {
                let &ConsCell { car: symbol, cdr, .. } =
                    into_type_or_error!(l : binding_pair => &ConsCell);
                let &ConsCell { car: value, .. } = into_type_or_error!(l : cdr => &ConsCell);
                scope.push((
                    into_type_or_error!(l : symbol => *const Symbol),
                    {
                        let r = l.evaluate(value);
                        bubble!(r);
                        r
                    }
                ));
            }
            l.new_scope(&scope);
            let mut res = Object::nil();
            for body_clause in &body {
                res = l.evaluate(*body_clause);
                bubble!(res);
            }
            l.end_scope();
            res
        },
        "setq" (symbol value &rest symbols values) -> {
            let n_args = unsafe { pop_bubble!(l).into_unchecked() };
            if ::math::oddp(n_args) {
                let e: Error = ErrorKind::WantedEvenArgCt.into();
                let e: RlispError = e.into();
                l.alloc(e)
            } else {
                let mut n_args: u32 = n_args as _;
                let mut res = Object::nil();
                while n_args > 1 {
                    n_args -= 2;
                    let sym = pop_bubble!(l);
                    let value = pop_bubble!(l);
                    let sym = into_type_or_error!(l : sym => *const Symbol);
                    res = l.evaluate(value);
                    bubble!(res);
                    l.set_symbol(sym, res);
                }
                res
            }
        },
        "quote" (x) -> {
            let n_args = pop_bubble!(l);
            if n_args != Object::from(1.0) {
                let e: Error =
                    ErrorKind::WrongArgsCount(
                        unsafe { n_args.into_unchecked() },
                        1, Some(1)
                    ).into();
                let e: RlispError = e.into();
                l.alloc(e)
            } else {
                pop_bubble!(l)
            }
        },
        "if" (predicate ifclause &rest elseclauses) -> {
            let n_args: u32 = unsafe { pop_bubble!(l).into_unchecked() };
            if n_args < 2 {
                let e: Error = ErrorKind::WrongArgsCount(n_args, 2, None).into();
                let e: RlispError = e.into();
                l.alloc(e)
            } else {
                let cond = pop_bubble!(l);
                let if_clause = pop_bubble!(l);
                let mut else_clauses = Vec::with_capacity(n_args as usize - 2);
                for _ in 0..(n_args - 2) {
                    else_clauses.push(pop_bubble!(l));
                }
                if bool::from({
                    let r = l.evaluate(cond);
                    bubble!(r);
                    r
                }) {
                    l.evaluate(if_clause)
                } else {
                    let mut res = Object::nil();
                    for clause in &else_clauses {
                        res = l.evaluate(*clause);
                    }
                    res
                }
            }
        },
        "defun" (name arglist &rest body) -> {
            let n_args: u32 = unsafe { pop_bubble!(l).into_unchecked() };
            if n_args < 3 {
                let e: Error = ErrorKind::WrongArgsCount(n_args, 3, None).into();
                let e: RlispError = e.into();
                l.alloc(e)
            } else {
                let name = pop_bubble!(l);
                let arglist = pop_bubble!(l);
                let mut body = Vec::with_capacity(n_args as usize - 2);
                for _ in 0..(n_args - 2) {
                    body.push(pop_bubble!(l));
                }
                let fun = l.alloc(
                    RlispFunc::from_body(body)
                        .with_name(name)
                        .with_arglist(arglist)
                );
                let name = into_type_or_error!(l : name => *const Symbol);
                l.set_symbol(name, fun);
                fun
            }
        },
        "defvar" (name value) -> {
            let n_args: u32 = unsafe { pop_bubble!(l).into_unchecked() };
            if n_args != 2 {
                let e: Error = ErrorKind::WrongArgsCount(n_args, 2, Some(2)).into();
                let e: RlispError = e.into();
                l.alloc(e)
            } else {
                let name = pop_bubble!(l);
                let val = pop_bubble!(l);
                let val = l.evaluate(val);
                bubble!(val);
                let name = into_type_or_error!(l : name => *const Symbol);
                l.set_symbol(name, val);
                val
            }
        },
        "catch-error" (statement &rest handlers) -> {
            let n_args: u32 = unsafe { pop_bubble!(l).into_unchecked() };
            info!("catch-error: called catch-error");
            let statement = pop_bubble!(l);

            let mut handlers = Vec::with_capacity(n_args as usize - 1);
            for _ in 0..(n_args - 1) {
                let arg = pop_bubble!(l);
                handlers.push(arg);
            }

            for handler in &handlers {
                info!("catch-error: with a handler {}", handler);
            }

            let res = l.evaluate(statement);

            info!("catch-error: {} evaluated to {}", statement, res);
            if let Some(e) = <Object as MaybeInto<&RlispError>>::maybe_into(res) {
                info!("catch-error: res is the error {}", e);
                let e = &e.error;
                info!("catch-error: e.error = {}", e);
                let e = l.error_name(e);
                info!("catch-error: the error is named {}", e);
                for handler in handlers {
                    let &ConsCell { car, cdr, .. } =
                        into_type_or_error!(l : handler => &ConsCell);
                    if (car == e) || (car == Object::t()) {
                        info!("catch-error: handler with car {} matched", car);
                        let &ConsCell { car, .. } =
                            into_type_or_error!(l : cdr => &ConsCell);
                        info!("will eval and return {}", car);
                        return l.evaluate(car);
                    }
                }
            }
            res
        },
        "lambda" (args &rest body) -> {
            let n_args: u32 = unsafe { pop_bubble!(l).into_unchecked() };
            if n_args < 2 {
                let e: Error = ErrorKind::WrongArgsCount(n_args, 2, None).into();
                let e: RlispError = e.into();
                l.alloc(e)
            } else {
                let arglist = pop_bubble!(l);
                let mut body = Vec::with_capacity(n_args as usize - 1);
                for _ in 0..(n_args - 1) {
                    body.push(pop_bubble!(l));
                }
                if (!arglist.nilp()) && (!<&ConsCell as FromObject>::is_type(arglist)) {
                    let e = RlispError::wrong_type(l.type_name(RlispType::Cons),
                                                   l.type_name(arglist.what_type()));
                    l.alloc(e)
                } else {
                    l.alloc(
                        RlispFunc::from_body(body)
                            .with_arglist(arglist)
                    )
                }
            }
        },
    }
}

pub fn make_builtins() -> RlispBuiltins {
    builtin_functions!{
        l = lisp;
        "numberp" (n) -> { n.numberp().into() },
        "consp" (c) -> { c.consp().into() },
        "cons" (car cdr) -> { l.alloc(ConsCell::new(car, cdr)) },
        "list" (&rest items) -> { items },
        "debug" (obj) -> { println!("{:?}", obj); obj },
        "print" (&rest objects) -> {
            if let Some(cons) = <&ConsCell as MaybeFrom<_>>::maybe_from(objects) {
                let mut count: i32 = 0;
                for obj in cons {
                    print!("{}", obj);
                    count += 1;
                }
                println!();
                count.into()
            } else {
                false.into()
            }
        },
        "eq" (first &rest objects) -> {
            if let Some(cons) = <&ConsCell as MaybeFrom<_>>::maybe_from(objects) {
                #[cfg_attr(feature = "cargo-clippy", allow(explicit_iter_loop))]
                for el in cons.into_iter() {
                    if !(first == el) {
                        return false.into();
                    }
                }
            } else if !objects.nilp() {
                let e: RlispError = RlispError::wrong_type(l.type_name(RlispType::Cons),
                                                      l.type_name(objects.what_type()));

                return l.alloc(e);
            }
            true.into()
        },
    }
}

pub fn builtin_vars() -> RlispBuiltinVars {
    builtin_vars! {
        "*pi*" = ::std::f64::consts::PI,
        "*e*" = ::std::f64::consts::E,
        "*infinity*" = ::std::f64::INFINITY,
    }
}
