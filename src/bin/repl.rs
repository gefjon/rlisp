extern crate rlisp;
use rlisp::lisp::Lisp;
use rlisp::repl;

fn main() {
    let mut lisp = Lisp::new();
    <Lisp as repl::stdio::StdIoRepl>::repl(&mut lisp).unwrap();
}
