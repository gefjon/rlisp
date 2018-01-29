use result::*;
use repl::{Rep, Repl};
use lisp::Lisp;
use std::{convert, mem};

pub struct VecRepl<R: AsMut<Lisp>> {
    lisp: R,
}

impl<R: convert::AsMut<Lisp>> convert::From<R> for VecRepl<R> {
    fn from(lisp: R) -> Self {
        Self { lisp }
    }
}

impl<R: convert::AsMut<Lisp>> Repl<R, Lisp> for VecRepl<R> {
    type Input = Vec<u8>;
    type Output = Vec<u8>;
    type Error = Vec<u8>;
    fn run(
        &mut self,
        input: &mut Self::Input,
        output: &mut Self::Output,
        _error: &mut Self::Error,
    ) -> Result<()> {
        let mut iter = input.iter().map(|n| *n).peekable();

        loop {
            let result = <Lisp as Rep>::rep(self.lisp.as_mut(), &mut iter);
            match result {
                Ok(Some(out)) => Self::write_out(out, output)?,
                Ok(None) => break,
                Err(err) => {
                    return Err(err);
                }
            }
        }
        Ok(())
    }
    fn write_out(out: String, output: &mut Self::Output) -> Result<()> {
        mem::replace(output, out.into_bytes());
        Ok(())
    }
    fn write_error(_err: Error, _error: &mut Self::Error) -> Result<()> {
        unreachable!()
    }
}
