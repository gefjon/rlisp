extern crate rlisp;
use rlisp::repl::stdio::StdIoRepl;

fn main() {
    let mut repl = StdIoRepl::new();
    repl.repl().unwrap();
}
