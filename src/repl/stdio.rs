use result::*;
use std::io;
use std::io::prelude::*;
use super::{Rep, Repl};
use lisp::Lisp;
use std::iter::Iterator;
use std::default::Default;
use std::convert;

// This is the struct that bin/repl.rs
// uses. StdIoRepl::default().repl().unwrap() will run an Rlisp REPL
// and exit either on I/O error or on EOF. Note: because it will not
// exit on an internal error, the stack can become deformed when an
// operation goes wrong.
pub struct StdIoRepl {
    lisp: Lisp,
}

impl Default for StdIoRepl {
    fn default() -> Self {
        Self {
            lisp: Lisp::default(),
        }
    }
}

impl convert::From<Lisp> for StdIoRepl {
    fn from(lisp: Lisp) -> Self {
        Self { lisp }
    }
}

impl convert::From<StdIoRepl> for Lisp {
    fn from(repl: StdIoRepl) -> Self {
        repl.lisp
    }
}

impl Repl<Lisp> for StdIoRepl {
    type Input = io::Stdin;
    type Output = io::Stdout;
    type Error = io::Stderr;
    fn run(
        &mut self,
        input: &mut Self::Input,
        output: &mut Self::Output,
        error: &mut Self::Error,
    ) -> Result<()> {
        let unwrap_ptr: fn(::std::result::Result<u8, _>) -> u8 =
            ::std::result::Result::<u8, _>::unwrap;
        let mut iter = input.bytes().map(unwrap_ptr).peekable();

        loop {
            self.prompt(output)?;
            let result = <Lisp as Rep>::rep(&mut self.lisp, &mut iter);
            match result {
                Ok(Some(out)) => Self::write_out(out, output)?,
                Ok(None) => break,
                Err(err) => Self::write_error(err, error)?,
            }
        }
        Ok(())
    }
    fn write_out(out: String, output: &mut Self::Output) -> Result<()> {
        write!(output, "{}\n", out)?;
        output.flush()?;
        Ok(())
    }
    fn write_error(err: Error, error: &mut Self::Error) -> Result<()> {
        write!(error, "ERROR: {}\n", err)?;
        error.flush()?;
        Ok(())
    }
    fn prompt(&mut self, output: &mut Self::Output) -> Result<()> {
        write!(output, "lisp> ")?;
        output.flush()?;
        Ok(())
    }
}
