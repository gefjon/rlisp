use result::*;
use super::{Rep, Repl};
use lisp::Lisp;
use std::default::Default;
use std::{boxed, convert, mem};
use std::iter::Iterator;

pub struct StringRepl<R: convert::AsMut<Lisp>> {
    lisp: R,
}

impl<R: convert::AsMut<Lisp>> convert::From<R> for StringRepl<R> {
    fn from(lisp: R) -> Self {
        Self { lisp }
    }
}

impl<R> Repl<R, Lisp> for StringRepl<R>
where
    R: convert::AsMut<Lisp>,
{
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
        mem::replace(output, out);
        Ok(())
    }
    fn write_error(_err: Error, _error: &mut Self::Error) -> Result<()> {
        unreachable!()
    }
}

impl Default for StringRepl<boxed::Box<Lisp>> {
    fn default() -> Self {
        Self {
            lisp: boxed::Box::new(Lisp::default()),
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
