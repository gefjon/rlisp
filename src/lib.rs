#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate log;

mod result {
    error_chain! {
        foreign_links {
            Io(::std::io::Error);
            Fmt(::std::fmt::Error);
            ParseFloat(::std::num::ParseFloatError);
            ParseInt(::std::num::ParseIntError);
            Utf8(::std::string::FromUtf8Error);
        }
        errors {
            UnclosedList {
                description("met EOF before a list was closed"),
                display("met EOF before a list was closed"),
            }
            UnclosedString {
                description("met EOF before a string was closed"),
                display("met EOF before a string was closed"),
            }
            UnexpectedEOF {
                description("met EOF before finished parsing"),
                display("met EOF before finished parsing"),
            }
            UnboundSymbol {
                description("attempted to access the value of an unbound symbol"),
                display("attempted to access the value of an unbound symbol"),
            }
        }
    }
}

mod reader;
mod evaluator;
pub mod types;
pub mod repl;
pub mod lisp;
pub mod list;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
