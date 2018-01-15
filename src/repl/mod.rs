use result::*;
use lisp::Lisp;
use types::*;

// stdio contains the REPL which reads from Stdin and prints to Stdout
pub mod stdio;

// Rep::rep(&mut Iterator<u8>) -> Result<String> is the forward-facing
// method of this trait. This trait should be accessed by a struct
// which owns a lisp::Lisp and which implements a way to create an
// Iterator<u8> (probably by io::Read::bytes()) and to print a string
// or an Err
pub trait Rep<V: Iterator<Item = u8>>
    : ::reader::Reader<V> + ::evaluator::Evaluator {
    fn read(&mut self, input: &mut V) -> Result<Option<Object>> {
        <Self as ::reader::Reader<V>>::read(self, input)
    }
    fn eval(&mut self, read: Object) -> Result<Object> {
        <Self as ::evaluator::Evaluator>::evaluate(self, read)
    }
    fn print(&self, evaled: Object) -> Result<String> {
        Ok(format!("{}\n", evaled))
    }
    fn rep(&mut self, input: &mut V) -> Result<String> {
        let read = <Self as Rep<V>>::read(self, input)?;
        if let Some(obj) = read {
            let evaled = self.eval(obj)?;
            self.print(evaled)
        } else {
            Ok(String::new())
        }
    }
}

impl<'read> Rep<::reader::StdioIter<'read>> for Lisp {}

#[cfg(test)]
mod tests {
    use result::*;
    use lisp::Lisp;
    use super::Rep;
    #[test]
    fn simple_print() {
        let input = String::from("'(foo bar)");
        let mut lisp = Lisp::new();
        let output = lisp.rep(input.clone()).unwrap();
        assert_eq!(input, output);
    }
}
