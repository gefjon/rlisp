use result::*;
use std::io;
use std::io::prelude::*;
use super::Rep;
use lisp::Lisp;

pub struct StdIoRepl {
    lisp: Lisp,
}

impl StdIoRepl {
    pub fn new() -> Self {
        Self { lisp: Lisp::new() }
    }
    fn prompt(stdout: &mut io::StdoutLock) -> Result<()> {
        stdout.write(b"lisp> ")?;
        stdout.flush()?;
        Ok(())
    }
    fn write_to_stdout(to_write: &str, stdout: &mut io::StdoutLock) -> Result<()> {
        stdout.write(to_write.as_bytes())?;
        stdout.flush()?;
        Ok(())
    }
    fn write_to_stderr(err: &str) -> Result<()> {
        let mut stderr = io::stderr();
        stderr.write(err.as_bytes())?;
        stderr.flush()?;
        Ok(())
    }
    pub fn repl(&mut self) -> Result<()> {
        let stdin = io::stdin();
        let lock = stdin.lock();
        let unwrap_ptr: fn(::std::result::Result<u8, _>) -> u8 =
            ::std::result::Result::<u8, _>::unwrap;
        let mut iter = lock.bytes().map(unwrap_ptr);

        let stdout = io::stdout();
        let mut stdoutlock = stdout.lock();

        'repl: loop {
            Self::prompt(&mut stdoutlock)?;
            let result = <Lisp as Rep<::reader::StdioIter>>::rep(&mut self.lisp, &mut iter);
            match result {
                Ok(output) => Self::write_to_stdout(&output, &mut stdoutlock)?,
                Err(err) => Self::write_to_stderr(&format!("{}", err))?,
            }
        }
    }
}
