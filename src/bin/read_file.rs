extern crate env_logger;
extern crate rlisp;
use rlisp::repl::vec_repl::VecRepl;
use rlisp::repl::Repl;
use std::{fs, io};
use std::io::prelude::*;
use rlisp::lisp;
use std::default::Default;

fn main() {
    let mut args = ::std::env::args();
    let _ = args.next(); // pop the executable name
    let mut l = lisp::Lisp::default();
    let mut vec_repl = VecRepl::from(&mut l);
    for filename in args {
        match fs::File::open(filename) {
            Ok(mut f) => {
                let mut contents = Vec::new();
                f.read_to_end(&mut contents).unwrap();
                let mut output = Vec::new();
                vec_repl
                    .run(&mut contents, &mut output, &mut Vec::new())
                    .unwrap();
                println!("{}", String::from_utf8(output).unwrap());
            }
            Err(e) => {
                let mut stderr = io::stderr();
                write!(stderr, "ERROR: read_file(): {}", e).unwrap();
            }
        }
    }
}
