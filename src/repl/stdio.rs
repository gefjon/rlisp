use result::*;
use std::io;
use std::io::prelude::*;
use super::Rep;
use lisp::Lisp;
use std::default::Default;

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

impl StdIoRepl {
    fn prompt(stdout: &mut io::StdoutLock) -> Result<()> {
        stdout.write_all(b"lisp> ")?;
        stdout.flush()?;
        Ok(())
    }
    fn write_to_stdout(to_write: &str, stdout: &mut io::StdoutLock) -> Result<()> {
        stdout.write_all(to_write.as_bytes())?;
        stdout.flush()?;
        Ok(())
    }
    fn write_to_stderr(err: &str) -> Result<()> {
        let mut stderr = io::stderr();
        stderr.write_all(err.as_bytes())?;
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

        loop {
            Self::prompt(&mut stdoutlock)?;
            let result = <Lisp as Rep<::reader::StdioIter>>::rep(&mut self.lisp, &mut iter);
            match result {
                Ok(output) => Self::write_to_stdout(&output, &mut stdoutlock)?,
                Err(err) => Self::write_to_stderr(&format!("ERROR: {}\n", err))?,
            }
        }
    }
}
