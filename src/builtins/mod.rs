use result::*;
use types::*;
use lisp;
use std::boxed::Box;
use lisp::allocate::AllocObject;

macro_rules! arglist {
    ($($arg:ident)*) => {
        {
            let mut arglist = Vec::new();
            $(arglist.push(String::from(stringify!($arg)));)*;
            arglist
        }
    };
    ($($arg:ident)* &optional $($oarg:ident)+) => {
        {
            let mut arglist = Vec::new();
            $(arglist.push(String::from(stringify!($arg)));)*;
            arglist.push(String::from("&optional"));
            $(arglist.push(String::from(stringify!($oarg)));)*;
            arglist
        }
    };
    ($($arg:ident)* &rest $rarg:ident) => {
        {
            let mut arglist = Vec::new();
            $(arglist.push(String::from(stringify!($arg)));)*;
            arglist.push(String::from("&rest"));
            arglist.push(String::from(stringify!($rarg)));
            arglist
        }
    };
    ($($arg:ident)* &rest $($rarg:ident)*) => {
        // Only special forms get to have multiple `&rest` args
        {
            let mut arglist = Vec::new();
            $(arglist.push(String::from(stringify!($arg)));)*;
            arglist.push(String::from("&rest"));
            $(arglist.push(String::from(stringify!($rarg)));)*;
            arglist
        }
    };
}

macro_rules! get_args {
    ($l:ident ; $_n_args:ident ; $($arg:ident)*) => {
        $(
            let $arg = <$crate::lisp::Lisp as $crate::lisp::stack_storage::Stack>
                ::pop($l)?;
        )*;
    };

    ($l:ident ; $n_args:ident ; $($arg:ident)* &optional $($oarg:ident)+) => {
        let mut consumed = 0;
        $(
            let $arg = <$crate::lisp::Lisp as $crate::lisp::stack_storage::Stack>
                ::pop($l)?;
            consumed += 1;
        )*;
        $(
            let $oarg = if consumed < $n_args {
                consumed += 1;
                <$crate::lisp::stack_storage::Stack>::pop($l)?
            } else {
                $crate::types::Object::nil()
            };
        )+;
    };

    ($l:ident ; $n_args:ident ; $($arg:ident)* &rest $rarg:ident) => {
        let mut consumed = 0;
        $(
            let $arg = <$crate::lisp::Lisp as $crate::lisp::stack_storage::Stack>::pop($l)?;
            consumed += 1;
        )*;
        let $rarg = {
            let mut head = $crate::types::Object::nil();
            while consumed < $n_args {
                consumed += 1;
                let conscell = $crate::types::ConsCell::new(
                    <$crate::lisp::Lisp as $crate::lisp::stack_storage::Stack>
                        ::pop($l)?, head);
                head = <$crate::lisp::Lisp as $crate::lisp::allocate::AllocObject>
                    ::alloc::<$crate::types::ConsCell>($l, conscell);
            }
            if head != $crate::types::Object::nil() {
                <$crate::lisp::Lisp as $crate::list::ListOps>
                    ::list_reverse($l, head.into_cons_or_error()?)
            } else {
                head
            }
        };
    };
}

macro_rules! builtin_function {
    ($l:ident $name:tt ($($arg:tt)*) -> $blk:block) => {
        {
            {
              (
                String::from(stringify!($name)),
                arglist!($($arg )*),
                Box::new(move |$l, _n_args| {
                    get_args!($l ; _n_args ; $($arg)*);
                    Ok($blk)
                })
             )
            }
        }
    };
}


macro_rules! special_form {
    ($l:ident $name:tt ($($arg:tt)*) -> $blk:block) => {
        {
            {
                (
                    String::from(stringify!($name)),
                    arglist!($($arg )*),
                    Box::new(move |$l| {
                        $blk
                    })
                )
            }
        }
    };
}

// This macro is the main part of this module. See make_builtins() for
// its use.  Each function consists of a string name, a list of
// identifiers which will have args bound to them, and a block which
// returns an Object. The block can use the `?` operator, but should
// do so sparingly
macro_rules! builtin_functions {
    (
        $l:tt = lisp;
        $($name:tt ($($arg:tt)*) -> $blk:block),* $(,)*
    ) => {{
        let mut v: $crate::builtins::RlispBuiltins = Vec::new();
        $(v.push(builtin_function!{$l $name ($($arg)*) -> $blk});)*;
        v
    }};
}

macro_rules! special_forms {
    (
        $l:tt = lisp;
        $($name:tt ($($arg:tt)*) -> $blk:block),* $(,)*
    ) => {{
        let mut v: $crate::builtins::RlispSpecialForms = Vec::new();
        $(v.push(special_form!{$l $name ($($arg)*) -> $blk});)*;
        v
    }};
}

pub type RlispBuiltinFunc = FnMut(&mut lisp::Lisp, usize) -> Result<Object>;
pub type RlispSpecialForm = FnMut(&mut lisp::Lisp) -> Result<Object>;
pub type Arglist = Vec<String>;
pub type Name = String;
pub type RlispBuiltinTuple = (Name, Arglist, Box<RlispBuiltinFunc>);
pub type RlispSpecialForms = Vec<(Name, Arglist, Box<RlispSpecialForm>)>;
pub type RlispBuiltins = Vec<RlispBuiltinTuple>;
        

pub fn make_special_forms() -> RlispSpecialForms {
    use evaluator::Evaluator;
    use lisp::stack_storage::Stack;
    special_forms!{
        l = lisp;
        cond (&rest clauses) -> {
            let n_args = l.pop()?.into_usize_or_error()?;
            debug!("called cond with {} args", n_args);
            let mut clauses = Vec::with_capacity(n_args);
            for _i in 0..n_args {
                debug!("popping arg {}", _i);
                let arg = l.pop()?;
                debug!("arg {} was {}", _i, arg);
                clauses.push(arg);
            }
            for clause in clauses.iter() {
                let &ConsCell { car, cdr, .. } = clause.into_cons_or_error()?;
                if bool::from(l.evaluate(car)?) {
                    let &ConsCell { car: cdrcar, .. } = cdr.into_cons_or_error()?;
                    return Ok(l.evaluate(cdrcar)?);
                }
            }
            Ok(Object::nil())
        },
        let (bindings &rest body) -> {
            let n_args = l.pop()?.into_usize_or_error()?;
            let bindings = l.pop()?;
            let mut body = Vec::with_capacity(n_args - 1);
            for _i in 0..(n_args - 1) {
                let arg = l.pop()?;
                body.push(arg);
            }
            let mut symbols_bound = Vec::new();
            for binding_pair in bindings.into_cons_or_error()?.into_iter() {
                let &ConsCell { car: symbol, cdr, .. } = binding_pair.into_cons_or_error()?;
                let &ConsCell { car: value, .. } = cdr.into_cons_or_error()?;
                symbol.into_symbol_mut_or_error()?.push(l.evaluate(value)?);
                symbols_bound.push(symbol);
            }
            let mut res = Object::nil();
            for body_clause in body.iter() {
                res = l.evaluate(*body_clause)?;
            }
            for symbol in symbols_bound.iter() {
                symbol.into_symbol_mut_or_error()?.pop();
            }
            Ok(res)
        },
        setq (symbol value &rest symbols values) -> {
            let n_args = l.pop()?;
            if ::math::oddp(n_args) {
                return Err(ErrorKind::WantedEvenArgCt.into());
            }
            let mut n_args = n_args.into_usize_or_error()?;
            let mut res = Object::nil();
            while (n_args > 1) {
                n_args -= 2;
                let sym = l.pop()?.into_symbol_mut_or_error()?;
                let value = l.pop()?;
                res = l.evaluate(value)?;
                sym.set(res);
            }
            Ok(res)
        },
        quote (x) -> {
            let n_args = l.pop()?;
            if !::math::num_equals(n_args, Object::from(1.0)) {
                Err(ErrorKind::WrongArgsCount(unsafe { n_args.into_usize_unchecked() }, 1, Some(1)).into())
            } else {
                Ok(l.pop()?)
            }
        }
    }
}

pub fn make_builtins() -> RlispBuiltins {
    builtin_functions!{
        l = lisp;
        numberp (n) -> { n.numberp().into() },
        consp (c) -> { c.consp().into() },
        cons (car cdr) -> { l.alloc(ConsCell::new(car, cdr)) },
        list (&rest items) -> { items },
        debug (obj) -> { println!("{:?}", obj); obj },
    }
}
