use result::*;
use types::*;
use lisp;
use lisp::stack_storage::Stack;
use std::boxed::Box;

// This macro is the main part of this module. See make_builtins() for
// its use.  Each function consists of a string name, a list of
// identifiers which will have args bound to them, and a block which
// returns an Object. The block can use the `?` operator, but should
// do so sparingly
macro_rules! builtin_functions {
    ($($name:expr ; ($($arg:ident) *) -> $blk:block),*) => {
         {
             let mut v: Vec<(String, Box<RlispBuiltinFunc>)> = Vec::new();
             $(v.push((
                 String::from($name),
                 Box::new(move |l: &mut lisp::Lisp| {
                     $(let $arg = l.pop()?;)*
                     return Ok($blk);
                 })
             ));)*
                 v
         }
     };
}

pub type RlispBuiltinFunc = FnMut(&mut lisp::Lisp) -> Result<Object>;
pub type RlispBuiltinTuple = (String, Box<RlispBuiltinFunc>);

pub fn make_builtins() -> Vec<(String, Box<RlispBuiltinFunc>)> {
    builtin_functions!{
        "numberp" ; (n) -> { n.numberp().into() },
        "consp" ; (c) -> { c.consp().into() }
    }
}
