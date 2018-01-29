use result::*;
use lisp::Lisp;
use types::*;
use std::iter::{Iterator, Peekable};
use std::default::Default;
use std::convert;

// stdio contains the REPL which reads from Stdin and prints to Stdout
pub mod stdio;
pub mod string_repl;
pub mod vec_repl;

pub trait Repl<R, L>
where
    R: convert::AsMut<L>,
    L: Rep,
{
    type Input;
    type Output;
    type Error;
    fn run(
        &mut self,
        input: &mut Self::Input,
        output: &mut Self::Output,
        error: &mut Self::Error,
    ) -> Result<()>;
    fn write_out(out: String, output: &mut Self::Output) -> Result<()>;
    fn write_error(err: Error, error: &mut Self::Error) -> Result<()>;
    fn prompt(&mut self, _output: &mut Self::Output) -> Result<()> {
        Ok(())
    }
}

// Rep::rep(&mut Iterator<u8>) -> Result<String> is the forward-facing
// method of this trait. This trait should be accessed by a struct
// which owns a lisp::Lisp and which implements a way to create an
// Iterator<u8> (probably by io::Read::bytes()) and to print a string
// or an Err
pub trait Rep: ::reader::Reader + ::evaluator::Evaluator {
    fn read<V: Iterator<Item = u8>>(&mut self, input: &mut Peekable<V>) -> Result<Option<Object>> {
        <Self as ::reader::Reader>::read(self, input)
    }
    fn eval(&mut self, read: Object) -> Object {
        let res = <Self as ::evaluator::Evaluator>::evaluate(self, read);
        if <&RlispError as conversions::FromObject>::is_type(res) {
            info!("evaluation errored; cleaning stack");
            <Self as ::lisp::stack_storage::Stack>::clean_stack(self);
        }
        res
    }
    fn print(&self, evaled: Object) -> String {
        format!("{}", evaled)
    }
    fn rep<V: Iterator<Item = u8>>(&mut self, input: &mut Peekable<V>) -> Result<Option<String>> {
        let read = <Self as Rep>::read(self, input)?;
        if let Some(obj) = read {
            let evaled = self.eval(obj);
            Ok(Some(self.print(evaled)))
        } else {
            Ok(None)
        }
    }
}

impl Rep for Lisp {}
