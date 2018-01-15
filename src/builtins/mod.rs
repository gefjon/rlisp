use result::*;
use types::*;
use lisp;
use lisp::stack_storage::Stack;
use std::boxed::Box;
use lisp::allocate::AllocObject;

// This macro is the main part of this module. See make_builtins() for
// its use.  Each function consists of a string name, a list of
// identifiers which will have args bound to them, and a block which
// returns an Object. The block can use the `?` operator, but should
// do so sparingly
macro_rules! builtin_functions {
    (
        let mut $l:ident = Lisp;
        $($name:expr ; ($($sym:expr;$arg:ident),*) -> $blk:block),*
    ) => {
         {
             let mut v: Vec<(Name, Arglist, Box<RlispBuiltinFunc>)> = Vec::new();
             $({
                 let arglist = vec![$(String::from($sym),)*];
                 let arg_ct = arglist.len();
                 v.push((
                 String::from($name),
                 arglist,
                 Box::new(move |$l: &mut lisp::Lisp| {
                     let num_passed = unsafe { $l.pop()?.into_usize_unchecked() };
                     if num_passed != arg_ct {
                         Err(ErrorKind::WrongArgsCount(arg_ct, num_passed).into())
                     } else {
                         $(let $arg = $l.pop()?;)*
                             Ok($blk)
                     }
                 })
                 ));})*
                 v
         }
     };
}

pub type RlispBuiltinFunc = FnMut(&mut lisp::Lisp) -> Result<Object>;
pub type Arglist = Vec<String>;
pub type Name = String;
pub type RlispBuiltinTuple = (Name, Arglist, Box<RlispBuiltinFunc>);

pub fn make_builtins() -> Vec<RlispBuiltinTuple> {
    builtin_functions!{
        let mut l = Lisp;
        "numberp" ; ("n";n) -> { n.numberp().into() },
        "consp" ; ("c";c) -> { c.consp().into() },
        "cons" ; ("car";car, "cdr";cdr) -> { l.alloc(ConsCell::new(car, cdr)) }
    }
}
