extern crate rlisp;
use rlisp::repl::stdio::StdIoRepl;

fn main() {
    let mut repl = StdIoRepl::default();
    repl.repl().unwrap();
}
