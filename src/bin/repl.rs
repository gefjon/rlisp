extern crate env_logger;
extern crate rlisp;
use rlisp::repl::stdio::StdIoRepl;

fn main() {
    env_logger::init();
    let mut repl = StdIoRepl::default();
    repl.repl().unwrap();
}
