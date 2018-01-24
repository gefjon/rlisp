use result::*;
use super::{Rep, Repl};
use lisp::Lisp;
use std::default::Default;
use std::{convert, mem};
use std::iter::Iterator;

pub struct StringRepl {
    lisp: Lisp,
}

impl convert::From<Lisp> for StringRepl {
    fn from(lisp: Lisp) -> Self {
        Self { lisp }
    }
}

impl convert::From<StringRepl> for Lisp {
    fn from(repl: StringRepl) -> Self {
        repl.lisp
    }
}

impl Repl<Lisp> for StringRepl {
    type Input = String;
    type Output = String;
    type Error = String;
    fn run(
        &mut self,
        input: &mut Self::Input,
        output: &mut Self::Output,
        _error: &mut Self::Error,
    ) -> Result<()> {
        let mut iter = input.bytes().peekable();

        loop {
            let result = <Lisp as Rep>::rep(&mut self.lisp, &mut iter);
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
        mem::replace(output, out);
        Ok(())
    }
    fn write_error(_err: Error, _error: &mut Self::Error) -> Result<()> {
        unreachable!()
    }
}

impl Default for StringRepl {
    fn default() -> Self {
        Self {
            lisp: Lisp::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use repl::Repl;
    #[test]
    fn one_plus_one() {
        let mut input = String::from("(+ 1 1)");
        let mut output = String::new();
        let mut error = String::new();
        let mut repl = StringRepl::default();
        repl.run(&mut input, &mut output, &mut error).unwrap();
        assert_eq!(output, "2");
    }
    #[test]
    fn multiple_in_sequence() {
        let mut input = String::from("(defvar x 1) (setq x (+ x x)) x");
        let mut output = String::new();
        let mut error = String::new();
        let mut repl = StringRepl::default();
        repl.run(&mut input, &mut output, &mut error).unwrap();
        assert_eq!(output, "2");
    }
}
