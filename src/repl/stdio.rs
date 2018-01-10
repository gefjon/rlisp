use result::*;
use std::io;
use std::io::prelude::*;
use super::Rep;

pub trait StdIoRepl: Rep<Vec<u8>> {
    fn read_from_stdin() -> Result<Vec<u8>> {
        Self::write_to_stdout("lisp> ")?;
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input)?;
        Ok(input.into_bytes())
    }
    fn write_to_stdout(to_write: &str) -> Result<()> {
        let mut stdout = io::stdout();
        stdout.write(to_write.as_bytes())?;
        stdout.flush()?;
        Ok(())
    }
    fn write_to_stderr(err: Vec<u8>) -> Result<()> {
        let mut stderr = io::stderr();
        stderr.write(&err)?;
        stderr.flush()?;
        Ok(())
    }
    fn repl(&mut self) -> Result<()> {
        'repl: loop {
            let input = Self::read_from_stdin()?;
            Self::write_to_stdout(&<Self as Rep<Vec<u8>>>::rep(self, input)?)?;
        }
    }
}

impl StdIoRepl for ::lisp::Lisp {}
