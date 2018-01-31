#![feature(allocator_api)]
#![feature(alloc)]
#![feature(unique)]
#![feature(trace_macros)]
#![feature(log_syntax)]
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate log;

extern crate alloc;

mod result {
    error_chain! {
        foreign_links {
            Io(::std::io::Error);
            Fmt(::std::fmt::Error);
            ParseFloat(::std::num::ParseFloatError);
            ParseInt(::std::num::ParseIntError);
            Utf8(::std::string::FromUtf8Error);
            StrUtf8(::std::str::Utf8Error);
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
            WrongArgsCount(got: usize, min: usize, max: Option<usize>) {
                description("wrong number of args passed to a function"),
                display("got {} args, but wanted between {} and {:?}", got, min, max),
            }
            RequiresArglist {
                description("the called function requires an arglist"),
                display("the called function requires an arglist but did not have one"),
            }
            WantedEvenArgCt {
                description("the called function wants an even number of arguments"),
                display("the called function wants an even number of arguments"),
            }
            OutOfArgs {
                description("ran out of args!"),
                display("ran out of args!"),
            }
        }
    }
}

#[macro_use]
mod rust_macros;

mod types;
mod builtins;
mod gc;
mod reader;
mod evaluator;
mod math;

pub mod repl;
pub mod lisp;
pub mod list;
