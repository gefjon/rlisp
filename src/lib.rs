#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate log;

mod result {
    error_chain! {
        foreign_links {
            Io(::std::io::Error);
            Fmt(::std::fmt::Error);
        }
        errors {
            UnclosedList {
                description("met EOF before a list was closed"),
                display("met EOF before a list was closed"),
            }
        }
    }
}

mod reader;
pub mod types;
mod repl;
pub mod lisp;
pub mod list;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
