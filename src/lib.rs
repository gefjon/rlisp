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
            Unbound {
                description("attempted to access the value of an unbound symbol"),
                display("attempted to access the value of an unbound symbol"),
            }
            StackUnderflow {
                description("attempted to pop off an empty stack"),
                display("attempted to pop off an empty stack"),
            }
            ImproperList {
                description("an improperly terminated list"),
                display("an improperly terminated list"),
            }
            NotAFunction {
                description("tried to evaluate a list whose car was not a funciton"),
                display("tried to evaluate a list whose car was not a funciton"),
            }
            WrongType(expected: ::types::RlispType, got: ::types::RlispType) {
                description("a type mismatch error"),
                display("Expected type {:?}, found type {:?}", expected, got),
            }
        }
    }
}

mod gc;
mod reader;
mod evaluator;
mod builtins;
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
