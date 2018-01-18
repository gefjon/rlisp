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
        $(v.push(builtin_function!{$l $name ($($arg)*) -> $blk});)*
            v
    }};
}

pub type RlispBuiltinFunc = FnMut(&mut lisp::Lisp, usize) -> Result<Object>;
pub type Arglist = Vec<String>;
pub type Name = String;
pub type RlispBuiltinTuple = (Name, Arglist, Box<RlispBuiltinFunc>);
pub type RlispBuiltins = Vec<RlispBuiltinTuple>;

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
