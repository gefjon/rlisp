extern crate env_logger;
#[macro_use]
extern crate log;
extern crate rlisp;
use rlisp::lisp::Lisp;
use rlisp::repl::string_repl::StringRepl;
use rlisp::repl::Repl;

const LISP_SOURCE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/lisp_source/try-catch.rlsp"
));

fn main() {
    env_logger::init();
    let mut lisp = Lisp::default();
    info!("finished creating lisp");
    let mut input = String::from(LISP_SOURCE);
    let mut repl = StringRepl::from(&mut lisp);
    let mut output = String::new();
    repl.run(&mut input, &mut output, &mut String::new())
        .unwrap();
    println!("{}", output);
}
