extern crate env_logger;
extern crate rlisp;
use rlisp::repl::stdio::StdIoRepl;
use rlisp::repl::Repl;
use std::io;

fn main() {
    env_logger::init();
    let mut repl = StdIoRepl::default();
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();
    repl.run(&mut stdin, &mut stdout, &mut stderr).unwrap();
}
